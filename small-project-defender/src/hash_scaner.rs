mod appconfig;

use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn hash_file(file_path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let hash = Sha256::digest(&buffer);
    Ok(format!("{:x}", hash))
}

pub fn scan_directory(hash_map: HashMap<String, String>, path: &str) -> HashMap<String, String> {
    let pathRef = Path::new(path);
    if pathRef.is_file() {
        let file_hash = hash_file(path).unwrap();
        hash_map.insert(path.to_string(), file_hash);
        return hash_map;
    }
    if !pathRef.is_dir() {
        return hash_map;
    }

    let dir = fs::read_dir(path)?;

    for entry in dir {
        let entry = entry?;
        let path = entry.path();

        hash_map = scan_directory(hash_map, &path.to_string());    
    }

    return hash_map;
}