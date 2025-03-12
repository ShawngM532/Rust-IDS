# Rust-Based Intrusion Detection System - Rustids

Aim: To Alert a specific System of incoming Attacks from external bad actors. We will be using the Rust Programming language 
Analyze packets for malicious patterns using Rust’s speed and safety.

Alert: dont forget to change the IP address used to bind to the listening port! it wont work otherwise! lookout for <insert-IP-&-Port number-here>.

## How to Use

First you need the files, use a simple git clone command and clone the current repository in your local computer.

```
git clone <link-to-git-repo>
```

Then run a simple build command within the directory

```
cargo build
```

Now run the command:

```
cargo run
```

For now you can test it with Nmap Scanning the specific IP address and port for your own Home Lab that you want to protect.

### Current version scans for NMAP scans and logs all the detected IP addresses and its ports into a log.txt file
