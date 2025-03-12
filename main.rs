use std::io::Write;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::signal;
use std::fs::OpenOptions;


/**
 * This is the entry point of the program. - the main function]
 * It creates a listener on a range of ports and listens for incoming connections.
 * When a connection is established, it spawns a new task to handle the connection.
 * The task reads the incoming request, checks if it is an Nmap scan, and logs the event if it is.
 * If the request is not an Nmap scan, it sends a response back to the client.
 * The server listens for a termination signal (like Ctrl+C) and shuts down gracefully when the signal is received.
 */
#[tokio::main]
async fn main() {
    let ip_address = "172.21.123.251";
    let ports = 1000..2000; // Example range of ports
    

    for port in ports {
        let addr = format!("{}:{}", ip_address, port);
        tokio::spawn(async move {
            if let Err(e) = start_listener(addr).await {
                eprintln!("Failed to bind to address: {}", e);
            }
        });
    }

    println!("Server listening on ports 4000 to 7999");

    
    signal::ctrl_c().await.expect("Failed to listen for termination signal");
    println!("Server shutting down");
}

/**
 * This function creates a TCP listener on the specified address and listens for incoming connections.
 * When a connection is established, it reads the incoming request, checks if it is an Nmap scan, and logs the event if it is.
 * If the request is not an Nmap scan, it sends a response back to the client.
 */
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
                        let log_message = format!("Nmap scan detected from {}\n", peer_addr);
                        println!("{}", log_message);
                        
                        if let Err(e) = log_to_file("log.txt", &log_message) {
                            eprintln!("Failed to write to log file: {}", e);
                        }
                    } else {
                        let _ = send_response(&mut stream, "HTTP/1.1 200 OK\r\n\r\nHello, World!").await;
                    }
                }
                Err(e) => {
                    eprintln!("Failed to read from client: {}", e);
                    if e.kind() == std::io::ErrorKind::ConnectionReset {
                        let log_message = format!("Alert: Nmap Scan detected from {}\n", peer_addr);
                        println!("{}", log_message);
                        
                        if let Err(e) = log_to_file("log.txt", &log_message) {
                            eprintln!("Failed to write to log file: {}", e);
                        }
                    }
                }
            }
        });
    }
}

/**
 * This function logs a message to a file. after using program check for a log.txt file in the same directory
 */
fn log_to_file(filename: &str, message: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)?;
    file.write_all(message.as_bytes())?;
    Ok(())
}

/**
 * This function reads the incoming request from the client.
 */
async fn receive_request(stream: &mut TcpStream) -> Result<String, std::io::Error> {
    let mut buf = [0; 1024];
    stream.read(&mut buf).await?;
    Ok(String::from_utf8_lossy(&buf[..]).to_string())
}

/**
 * This function sends a response back to the client.
 */
async fn send_response(stream: &mut TcpStream, response: &str) {
    let data = response.as_bytes();
    stream.write_all(data).await.expect("Failed to write to client");
}

/**
 * This function checks if the incoming request is an Nmap scan.
 * It looks for specific patterns in the request that are commonly associated with Nmap scans.
 */
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
}