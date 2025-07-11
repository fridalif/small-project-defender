use crate::prelude::*;
use std::sync::{mpsc, Arc, Mutex};
use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use tokio::runtime::Runtime;

async fn serve_tcp_client(config: Arc<AppConfig>, rx: mpsc::Receiver<HashMap<String, HashMap<String, String>>>) {
    
}

pub fn serve(config: Arc<AppConfig>, rx: mpsc::Receiver<HashMap<String, HashMap<String, String>>>) {
    let rt = Runtime::new().unwrap();
    rt.block_on(serve_tcp_client(config, rx));
}