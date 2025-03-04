use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, SocketAddr};

// Function to detect Nmap scan
fn detect_nmap_scan(request: &str) -> bool {
    // Common Nmap scan characteristics
    let nmap_patterns = [
        "Nmap", // Nmap version detection
        "libwww-perl", // Nmap HTTP library
        "Mozilla/5.0 (compatible; Nmap Scripting Engine; http://nmap.org/book/nse.html)", // Nmap NSE
    ];

    for pattern in &nmap_patterns {
        if request.contains(pattern) {
            return true;
        }
    }

    false
}

// Function to send back the response to the client
fn send_response(mut stream: TcpStream, response: &str) {
    let data = response.as_bytes();
    stream.write(data).expect("Failed to write to client");
}

// Function to receive the request from the client
fn receive_request(mut stream: TcpStream) -> Result<String, std::io::Error> {
    let mut buf = [0; 1024];
    match stream.read(&mut buf) {
        Ok(_) => Ok(String::from_utf8_lossy(&buf[..]).to_string()),
        Err(e) => Err(e),
    }
}

fn main() {
    // Create a listener on port 8080
    let listener = TcpListener::bind("<insert-IP-&-Port number-here>").expect("Failed to create listener");
    println!("Server listening on port 8080");

    // Accept incoming connections
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let peer_addr = stream.peer_addr().expect("Failed to get peer address");
                println!("Connection established from {}", peer_addr);

                match receive_request(stream.try_clone().expect("Failed to clone stream")) {
                    Ok(request) => {
                        println!("Request: {}", request);

                        if detect_nmap_scan(&request) {
                            println!("Nmap scan detected from {}", peer_addr);
                        } else {
                            send_response(stream, "HTTP/1.1 200 OK\r\n\r\nHello, World!");
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to read from client: {}", e);
                        if e.kind() == std::io::ErrorKind::ConnectionReset {
                            println!("Alert: Nmap Scan detected from {peer_addr}");
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to establish connection: {}", e);
            }
        }
    }
}
