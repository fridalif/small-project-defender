use sha2::{Digest, Sha256};
use std::fs::File;
use rand::Rng;
use std::io::prelude::*;
use std::collections::HashMap;
use std::path::Path;
use crate::prelude::*;

fn hash_file(file_path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let hash = Sha256::digest(&buffer);
    Ok(format!("{:x}", hash))
}

fn scan_directory(hash_map: HashMap<String, String>, path: &str, exceptions: Vec<String>) -> (HashMap<String, String>, Vec<String>) {
    let pathRef = Path::new(path);
    if pathRef.is_file() {
        let file_hash = hash_file(path).unwrap();
        hash_map.insert(path.to_string(), file_hash);
        return (hash_map, exceptions);
    }
    if !pathRef.is_dir() {
        return (hash_map, exceptions);
    }

    let dir = fs::read_dir(path)?;

    for entry in dir {
        let entry = entry?;
        let path = entry.path();
        if exceptions.contains(&path.to_string()) {
            continue;
        }
        hash_map = scan_directory(hash_map, &path.to_string(), exceptions);    
    }

    return (hash_map, exceptions);
}

pub fn init_origins_hash_scaner(config: appconfig::AppConfig) -> HashMap<String, String> {
    let mut origins: HashMap<String, String> = HashMap::new();
    for dir in config.hash_scaner.directories {
        (origins, exceptions) = scan_directory(origins, &dir, config.hash_scaner.exceptions);
    }
    return origins;
}

pub fn schedule_hash_scaner(origins: HashMap<String, String>, config: std::sync::Arc<AppConfig>) {
    let mut exceptions = config.hash_scaner.exceptions.clone();
    let temp_dirs = config.hash_scaner.directories.clone();
    while true {
        let temp_cooldown = config.hash_scaner.cooldown;
        let random_seconds = rand::thread_rng().gen_range(0..temp_cooldown);
        std::thread::sleep(std::time::Duration::from_secs(random_seconds));
        
        let mut new_origins: HashMap<String, String> = HashMap::new();
        for dir in temp_dirs.iter() {
            (new_origins, exceptions) = scan_directory(origins, &dir, exceptions);    
        }
        for (key, value) in new_origins {
            if !origins.contains_key(&key) {
                println!("New file detected: {} with hash: {}", key, value);
                continue
            }
            if origins.get(&key) != Some(&value) {
                println!("File changed: {} with hash: {}", key, value);
                continue
            }
        }
    }   
}