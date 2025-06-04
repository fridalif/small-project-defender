use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::sync::Arc;
use std::io::prelude::*;
use std::collections::{HashMap};
use std::path::Path;
use crate::prelude::*;
use glob::Pattern;

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

pub fn init_origins_hash_scaner(config: Arc<AppConfig>) -> HashMap<String, String> {
    let mut origins: HashMap<String, String> = HashMap::new();
    let mut exceptions = config.hash_scaner.exceptions.clone();
    for dir in config.hash_scaner.directories.clone() {
        (origins, exceptions) = scan_directory(origins, &dir, exceptions);
    }
    return origins;
}

pub fn schedule_hash_scaner(origins: HashMap<String, String>, config: Arc<AppConfig>) {
    let mut exceptions = config.hash_scaner.exceptions.clone();
    let dirs = config.hash_scaner.directories.clone();
    let mut new_hashes = origins.clone();
    loop {
        println!("Scanning directories...");
        let temp_cooldown = config.hash_scaner.cooldown;
        let random_seconds = rand::random_range(0..temp_cooldown);
        std::thread::sleep(std::time::Duration::from_secs(random_seconds));
        for dir in dirs.iter() {
            (new_hashes, exceptions) = scan_directory(new_hashes, dir.as_str(), exceptions)
        }
        for key in new_hashes.keys() {
            if !origins.contains_key(key) {
                println!("New file found: {}", key);
                continue;
            }
            if origins.get(key).unwrap() != new_hashes.get(key).unwrap() {
                println!("File changed: {}", key);
                continue;
            }
        }
    }   
}