use std::env;
use std::fs::File;
use std::io::{self, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

const DEFAULT_SERVER_IP: &str = "0.0.0.0"; // Listen on all interfaces
const DEFAULT_PORT: u16 = 8888;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer).unwrap();
    let file_name = String::from_utf8_lossy(&buffer[..bytes_read]).trim().to_string();
    
    println!("Requesting file: {}", file_name);

    match File::open(&file_name) {
        Ok(file) => {
            let mut reader = BufReader::new(file);
            let mut contents = Vec::new();
            reader.read_to_end(&mut contents).unwrap();
            stream.write_all(&contents).unwrap();
        }
        Err(_) => {
            let error_message = format!("Error: File {} not found", file_name);
            stream.write_all(error_message.as_bytes()).unwrap();
        }
    }
}

fn start_server(ip: &str, port: u16) {
    let listener = TcpListener::bind(format!("{}:{}", ip, port)).unwrap();
    println!("Server listening on {}:{}", ip, port);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || handle_client(stream));
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}

fn request_file(ip: &str, port: u16, file_name: &str) -> io::Result<Vec<u8>> {
    let mut stream = TcpStream::connect(format!("{}:{}", ip, port))?;
    stream.write_all(file_name.as_bytes())?;
    
    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer)?;
    
    Ok(buffer)
}

fn save_file(file_name: &str, contents: &[u8]) -> io::Result<()> {
    let mut file = File::create(file_name)?;
    file.write_all(contents)?;
    Ok(())
}

fn start_client(ip: &str, port: u16, file_name: &str) {
    match request_file(ip, port, file_name) {
        Ok(contents) => {
            if let Err(e) = save_file(&("received_".to_owned() + file_name), &contents) {
                eprintln!("Error saving file: {}", e);
            } else {
                println!("File received and saved as: received_{}", file_name);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <server|client> [<ip> <port> <file_name>]", args[0]);
        return;
    }

    let ip = if args.len() > 2 { &args[2] } else { DEFAULT_SERVER_IP };
    let port = if args.len() > 3 { args[3].parse().unwrap_or(DEFAULT_PORT) } else { DEFAULT_PORT };

    match args[1].as_str() {
        "server" => start_server(ip, port),
        "client" => {
            if args.len() < 4 {
                eprintln!("Usage for client: {} client <ip> <port> <file_name>", args[0]);
                return;
            }
            start_client(ip, port, &args[4]);
        }
        _ => eprintln!("Invalid argument: use 'server' or 'client'"),
    }
}
