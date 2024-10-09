mod server;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs::File;
use std::io::BufReader;
use std::thread;

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

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server listening on port 7878");

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
