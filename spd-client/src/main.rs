mod appconfig;
mod ssh_detector;
mod hash_scaner;
mod port_detector;
mod connection;
mod prelude {
    pub use crate::appconfig::AppConfig;
}

use std::sync::{mpsc, Arc, Mutex};
use std::collections::HashMap;
use std::thread;
use crate::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let config = Arc::new(AppConfig::new("../etc/config.yaml".to_string())?);
    let (tx, rx) = mpsc::channel();
    //let mut origins_hash_scaner = Arc::new(Mutex::new(HashMap::new()));
    //if config.hash_scaner.on {
    //    let config_clone = std::sync::Arc::clone(&config);
    //    origins_hash_scaner = hash_scaner::init_origins_hash_scaner(std::sync::Arc::clone(&config_clone));
    //    let origins_hash_scaner = std::sync::Arc::clone(&origins_hash_scaner);
    //    let tx = tx.clone();
    //    thread::spawn(move || hash_scaner::schedule_hash_scaner(std::sync::Arc::clone(&origins_hash_scaner), std::sync::Arc::clone(&config_clone), tx.clone()));
    //}
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
    {
        //let origins_hash_scaner = std::sync::Arc::clone(&origins_hash_scaner);
        let config_clone = std::sync::Arc::clone(&config);
        thread::spawn(move || connection::serve(std::sync::Arc::clone(&config_clone), rx));
    }

    Ok(())  
}




