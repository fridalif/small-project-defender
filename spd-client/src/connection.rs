use crate::{cryptography, prelude::*};
use std::sync::{mpsc, Arc, Mutex};
use std::collections::HashMap;
use std::net::TcpStream;


fn serve_tcp_client(config: Arc<AppConfig>, rx: mpsc::Receiver<HashMap<String, HashMap<String, String>>>) {
    let mut connected = false;
    let mut stream  = match TcpStream::connect_timeout(format!("{}:{}", config.socket.host, config.socket.port), Duration::from_secs(5)) {
        Ok(stream) => {
            connected = true;
            println!("Connected to {}", format!("{}:{}", config.socket.host, config.socket.port));
            stream
        },
        Err(e) => {
            connected = false;
            eprintln!("Failed to connect to {}: {}", format!("{}:{}", config.socket.host, config.socket.port), e);
            return Err(e.into());
        }
    };
    loop {
        match rx.recv_timeout(Duration::from_secs(5)) {
            Ok(message) => { 

            },
            Err(mpsc::RecvTimeoutError::Timeout) => { 
                continue; 
            },
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                break; 
            }
        }
    }
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