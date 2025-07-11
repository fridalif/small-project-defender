use crate::{cryptography, prelude::*};
use std::sync::{mpsc, Arc, Mutex};
use std::collections::HashMap;
use tokio::net::TcpStream;
use tokio::runtime::Runtime;


async fn serve_tcp_client(config: Arc<AppConfig>, rx: mpsc::Receiver<HashMap<String, HashMap<String, String>>>) {
    let mut stream = match TcpStream::connect(format!("{}:{}", config.socket.host, config.socket.port)).await {
        Ok(stream) => {
            println!("Connected to {}", format!("{}:{}", config.socket.host, config.socket.port));
            stream
        },
        Err(e) => {
            eprintln!("Failed to connect to {}: {}", format!("{}:{}", config.socket.host, config.socket.port), e);
            return;
        }
    };
    loop {

    }
}

pub fn serve(config: Arc<AppConfig>, rx: mpsc::Receiver<HashMap<String, HashMap<String, String>>>) {
    let rt = Runtime::new().unwrap();
    rt.block_on(serve_tcp_client(config, rx));
}