mod client;
use std::io::{self, Write, Read};
use std::net::TcpStream;
use std::fs::File;

fn request_file(file_name: &str) -> io::Result<Vec<u8>> {
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;
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

fn main() {
    let file_name = "example.txt"; // Change to the file you want to request
    match request_file(file_name) {
        Ok(contents) => {
            if let Err(e) = save_file("received_" + file_name, &contents) {
                eprintln!("Error saving file: {}", e);
            } else {
                println!("File received and saved as: received_{}", file_name);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
