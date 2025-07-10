mod appconfig;
mod ssh_detector;
mod hash_scaner;
mod port_detector;
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
        let config_clone = std::sync::Arc::clone(&config);
        let origins_hash_scaner = hash_scaner::init_origins_hash_scaner(std::sync::Arc::clone(&config_clone));
        let tx = tx.clone();
        thread::spawn(move || hash_scaner::schedule_hash_scaner(origins_hash_scaner, std::sync::Arc::clone(&config_clone), tx.clone()));
    }
    if config.ssh_detector.check_auth_on {
        let config_clone = std::sync::Arc::clone(&config);
        let tx = tx.clone();
        thread::spawn(move || ssh_detector::ssh_auth_log_watcher(std::sync::Arc::clone(&config_clone), tx.clone()));
    }
    if config.ssh_detector.check_journalctl_on {
        let config_clone = std::sync::Arc::clone(&config);
        let tx = tx.clone();
        thread::spawn(move || ssh_detector::journalctl_watcher(std::sync::Arc::clone(&config_clone), tx.clone()));
    }
    if config.port_detector.on {
        let config_clone = std::sync::Arc::clone(&config);
        let tx = tx.clone();
        thread::spawn(move || port_detector::port_detector(std::sync::Arc::clone(&config_clone), tx.clone()));
    }
    for message in rx {
        println!("{}", message.keys().next().unwrap());
    }

    Ok(())  
}




