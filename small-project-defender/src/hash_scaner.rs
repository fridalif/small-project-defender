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

pub fn scan_directory(config: &appconfig::AppConfig) -> HashMap<String, String> {
    
}