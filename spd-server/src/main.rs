mod prelude {
    pub use crate::appconfig::AppConfig;
    pub use crate::cryptography;
}
use crate::prelude::*;
use std::sync::Arc;
use tokio::net::TcpListener;


#[tokio::main]
async fn main() {
    let config = Arc::new(AppConfig::new("../etc/config.yaml".to_string())?);    
}
