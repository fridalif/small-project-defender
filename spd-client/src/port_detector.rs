use crate::prelude::*;
use std::sync::{Arc, mpsc};
use std::process::Command;
use std::collections::HashMap;
use std::str;

pub fn port_detector(config: Arc<AppConfig>, tx: mpsc::Sender<HashMap<String, String>>) {
    loop {
        let temp_cooldown = config.port_detector.cooldown;
        let random_seconds = rand::random_range(0..temp_cooldown);
        std::thread::sleep(std::time::Duration::from_secs(random_seconds));
        let output = Command::new("ss")
            .arg("-tuln")
            .output()
            .map_err(|e| format!("Failed to execute ss: {}", e)).unwrap();

        if !output.status.success() {
            continue;
        }
        let stdout = str::from_utf8(&output.stdout)
        .map_err(|e| format!("Invalid output: {}", e)).unwrap();

        let ports: Vec<String> = stdout
        .lines()
        .skip(1)
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            parts.get(4)
                .and_then(|addr| addr.split(':').last())
                .map(|port| format!("Port: {}, Protocol: {}", port, parts[0]))
        }).collect();
        
        let mut alerts_map = HashMap::new();
        for port in ports {
            if !config.port_detector.legit_ports.contains(&port) {
                alerts_map.insert("info".to_string(), format!("Detected not legit port: {}", port));
            }
        }
        let mut response_map = HashMap::new();
        response_map.insert("port_detector".to_string(), alerts_map);
        tx.send(response_map).unwrap_or_default();
    }
}