use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::sync::{Arc,Mutex};
use std::io::prelude::*;
use std::collections::{HashMap};
use std::path::Path;
use crate::prelude::*;
use glob::Pattern;
use std::sync::mpsc;

fn hash_file(file_path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let hash = Sha256::digest(&buffer);
    Ok(format!("{:x}", hash))
}

fn is_exception(path: &str, exceptions: Vec<String>) -> bool {
    for exception in exceptions {
        if let Ok(pattern) = Pattern::new(&exception) {
            if pattern.matches(path) {
                return true;
            }
        }
    }
    false
}

fn scan_directory(hash_map: HashMap<String, String>, path: &str, exceptions: Vec<String>) -> (HashMap<String,String>, Vec<String>) {
    let path_ref = Path::new(path);
    let mut exceptions = exceptions;
    let mut hash_map = hash_map;
    if path_ref.is_file() {
        let file_hash = hash_file(path).unwrap();
        hash_map.insert(path.to_string(), file_hash);
        return (hash_map, exceptions);
    }
    if !path_ref.is_dir() {
        return (hash_map, exceptions);
    }

    let dir = fs::read_dir(path).unwrap();

    for entry in dir {
        let entry = entry.unwrap();
        let path = entry.path();
        if is_exception(path.to_str().unwrap(), exceptions.clone()) { 
            continue; 
        }
        (hash_map, exceptions) = scan_directory(hash_map, &path.to_str().unwrap(), exceptions);    
    }

    return (hash_map, exceptions);
}

pub fn init_origins_hash_scaner(config: Arc<AppConfig>, origins: Arc<Mutex<HashMap<String, String>>>) -> Arc<Mutex<HashMap<String, String>>> {
    let mut origins_map: HashMap<String, String> = HashMap::new();
    let mut exceptions = config.hash_scaner.exceptions.clone();
    for dir in config.hash_scaner.directories.clone() {
        (origins, exceptions) = scan_directory(origins, &dir, exceptions);
    }
    return Arc::new(Mutex::new(origins));
}

pub fn schedule_hash_scaner(origins: Arc<Mutex<HashMap<String, String>>>, config: Arc<AppConfig>, tx: mpsc::Sender<HashMap<String, HashMap<String, String>>>) {
    let mut exceptions = config.hash_scaner.exceptions.clone();
    let dirs = config.hash_scaner.directories.clone();

    loop {
        let temp_cooldown = config.hash_scaner.cooldown;
        let random_seconds = rand::random_range(0..temp_cooldown);
        std::thread::sleep(std::time::Duration::from_secs(random_seconds));

        let mut new_hashes = origins.lock().unwrap().clone();
        for dir in dirs.iter() {
            (new_hashes, exceptions) = scan_directory(new_hashes, dir.as_str(), exceptions);
        }
        let mut alerts_map =  HashMap::new(); 
        for key in new_hashes.keys() {
            if !origins.lock().unwrap().contains_key(key) {
                alerts_map.insert(key.to_string(), format!("Detected new file: {}", new_hashes.get(key).unwrap()));
                continue;
            }
            if origins.lock().unwrap().get(key).unwrap() != new_hashes.get(key).unwrap() {
                alerts_map.insert(key.to_string(), format!("Detected modified file: {}", new_hashes.get(key).unwrap()));
                continue;
            }
            
        }
        if alerts_map.keys().len() != 0 {
            let mut response_map = HashMap::new();
            response_map.insert("hash_scaner".to_string(), alerts_map);
            tx.send(response_map).unwrap();
        }
    }   
}