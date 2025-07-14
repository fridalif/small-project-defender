use crate::prelude::*;
use tokio::net::TcpListener;


async fn serve(socket, address, key: Arc<String>) {

}

pub async fn init_server(config: Arc<AppConfig>) {
    let server = TcpListener::bind((config.server_settings.ip, config.server_settings.port)).await.unwrap();
    println!("Server started at {}:{}", config.server_settings.ip, config.server_settings.port);

}