use std::io::{Read, Write};
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::signal;

#[tokio::main]
async fn main() {
    let ip_address = "172.21.123.251";
    let ports = 4000..8000; // Example range of ports

    for port in ports {
        let addr = format!("{}:{}", ip_address, port);
        tokio::spawn(async move {
            if let Err(e) = start_listener(addr).await {
                eprintln!("Failed to bind to address: {}", e);
            }
        });
    }

    println!("Server listening on ports 4000 to 7999");

    // Wait for a termination signal (like Ctrl+C)
    signal::ctrl_c().await.expect("Failed to listen for termination signal");
    println!("Server shutting down");
}

async fn start_listener(addr: String) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(&addr).await?;
    println!("Listening on {}", addr);

    loop {
        let (mut stream, peer_addr) = listener.accept().await?;
        println!("Connection established from {}", peer_addr);

        tokio::spawn(async move {
            match receive_request(&mut stream).await {
                Ok(request) => {
                    println!("Request: {}", request);

                    if detect_nmap_scan(&request) {
                        println!("Nmap scan detected from {}", peer_addr);
                    } else {
                        send_response(&mut stream, "HTTP/1.1 200 OK\r\n\r\nHello, World!").await;
                    }
                }
                Err(e) => {
                    eprintln!("Failed to read from client: {}", e);
                    if e.kind() == std::io::ErrorKind::ConnectionReset {
                        println!("Alert: Nmap Scan detected from {}", peer_addr);
                    }
                }
            }
        });
    }
}

async fn receive_request(stream: &mut TcpStream) -> Result<String, std::io::Error> {
    let mut buf = [0; 1024];
    stream.read(&mut buf).await?;
    Ok(String::from_utf8_lossy(&buf[..]).to_string())
}

async fn send_response(stream: &mut TcpStream, response: &str) {
    let data = response.as_bytes();
    stream.write_all(data).await.expect("Failed to write to client");
}

fn detect_nmap_scan(request: &str) -> bool {
    let nmap_patterns = [
        "Nmap",
        "libwww-perl",
        "Mozilla/5.0 (compatible; Nmap Scripting Engine; http://nmap.org/book/nse.html)",
    ];

    for pattern in &nmap_patterns {
        if request.contains(pattern) {
            return true;
        }
    }

    false
<<<<<<< HEAD
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
=======
}
>>>>>>> d1d524f (modified it to scan for a wide range of ports and scans all of them at once.)
