use crate::{cryptography, prelude::*};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::sync::{mpsc, Arc, Mutex};
use std::collections::HashMap;
use std::net::TcpStream;
use std::time::SystemTime;


fn format_message(module: &str,message: HashMap<String, String>) -> String {
    if module == "port_detector" || module == "ssh_detector" {
        return format!("Module:{}\nMessage:{}\n", module, message["info"])
    }
    return format!("Unknown module:{}\nUnknown message:{}\n", module, message)
}

fn message_after_reconnect(stream: &mut TcpStream, message: &str, key: &str,) -> String {
    let mut buffer = Vec<u8>::new();
    let encrypted_message = cryptography::encrypt(key.as_bytes(), message.as_bytes());
    stream.write_all(message.as_bytes()).unwrap_or_default();
    stream.read(&mut buffer).unwrap_or_default();
    let decrypted_bytes = cryptography::decrypt(key.as_bytes(), &buffer);
    return String::from_utf8(decrypted_bytes).unwrap();
}

fn serve_tcp_client(config: Arc<AppConfig>, rx: mpsc::Receiver<HashMap<String, HashMap<String, String>>>) {
    let output_file = match OpenOptions::new()
        .create(true)
        .append(true)
        .write(true)
        .open(config.socket.spare_log_file)
        {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Failed to create or open file: {}", e);
                return;
            }
        };

    let mut connected = false;
    let mut stream  = match TcpStream::connect_timeout(format!("{}:{}", config.socket.host, config.socket.port), Duration::from_secs(5)) {
        Ok(stream) => {
            connected = true;
            println!("Connected to {}", format!("{}:{}", config.socket.host, config.socket.port));
            stream
        },
        Err(e) => {
            connected = false;
            println!("Failed to connect to {}: {}", format!("{}:{}", config.socket.host, config.socket.port), e);
        }
    };

    let mut current_key = String::new();
    current_key = message_after_reconnect(&mut stream, message, &config.socket.init_secret);
    
    loop {
        match rx.recv_timeout(Duration::from_secs(5)) {
            Ok(message) => { 
                if !connected {
                    stream  = match TcpStream::connect_timeout(format!("{}:{}", config.socket.host, config.socket.port), Duration::from_secs(5)) {
                        Ok(stream) => {
                            connected = true;
                            println!("Connected to {}", format!("{}:{}", config.socket.host, config.socket.port));
                            current_key = message_after_reconnect(&mut stream, message, &config.socket.init_secret);
                            stream
                        },
                        Err(e) => {
                            connected = false;
                            println!("Failed to connect to {}: {}", format!("{}:{}", config.socket.host, config.socket.port), e);
                        }
                    };                 
                }
                let module = message.keys().next().unwrap();
                let sending_message = format_message(module, message);
                if connected {                    
                    let encrypted_message = cryptography::encrypt(current_key.as_bytes(), sending_message.as_bytes());
                    stream.write_all(encrypted_message.as_bytes()).unwrap_or_else(|_| {
                        eprintln!("Failed to send message"); 
                        connected=false;
                    });
                }
                writeln!(output_file, "{}|{}", SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap_or_default().as_secs(), sending_message)
            },
            Err(mpsc::RecvTimeoutError::Timeout) => { 
                continue; 
            },
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                break; 
            }
        }
    }
    writeln!(output_file, "{}|{}", SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap_or_default().as_secs(), "Connectons closed");
}

pub fn serve(config: Arc<AppConfig>, rx: mpsc::Receiver<HashMap<String, HashMap<String, String>>>) {
    let thread_handlers = !vec();
    {
        let config = config.clone();
        let rx = rx.clone();
        let handle = thread::spawn(move || serve_tcp_client(config, rx));
        thread_handlers.push(handle);
    }
    for handle in thread_handlers {
        handle.join().unwrap();
    }
}