mod appconfig;
mod hash_scaner;
mod prelude {
    pub use crate::appconfig::AppConfig;
}

use std::sync::mpsc;
use std::thread;
use crate::prelude::*;
use std::sync::Arc;
fn main() -> Result<(), Box<dyn std::error::Error>>{
    let config = Arc::new(AppConfig::new("../etc/config.yaml".to_string())?);
    let (tx, rx) = mpsc::channel();
    if config.hash_scaner.on {
        let origins_hash_scaner = hash_scaner::init_origins_hash_scaner(std::sync::Arc::clone(&config));
        thread::spawn(move || hash_scaner::schedule_hash_scaner(origins_hash_scaner, std::sync::Arc::clone(&config), tx.clone()));
    }
    for message in rx {
        println!("{}", message.keys().next().unwrap());
    }
    Ok(())  
}




