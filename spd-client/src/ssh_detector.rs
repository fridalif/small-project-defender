use inotify::{Inotify, WatchMask};
use regex::Regex;
use crate::prelude::*;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};
use std::sync::{Arc, mpsc};
use std::collections::HashMap;
use std::process::Command;

pub fn ssh_auth_log_watcher(config: Arc<AppConfig>, tx: mpsc::Sender<HashMap<String, HashMap<String, String>>>) {
    let mut inotify = Inotify::init().unwrap();

    inotify.watches().add(
        &config.ssh_detector.log_file,
        WatchMask::MODIFY,
    ).unwrap();

    let mut buffer = [0u8,255];
    let re = Regex::new(r"sshd\[(\d+)\].*Accepted.*for (\w+) from ([\d\.]+)").unwrap();

    loop {
        let events = inotify.read_events_blocking(&mut buffer).unwrap();
        
        for _event in events {
            let file = OpenOptions::new().read(true).open(&config.ssh_detector.log_file).unwrap();
            let reader = BufReader::new(file);
            let mut alerts_map = HashMap::new();
            for line in reader.lines() {
                let line = line.unwrap();
                
                if let Some(captures) = re.captures(&line) {
                    let pid = captures.get(1).unwrap().as_str();
                    let user = captures.get(2).unwrap().as_str();
                    let ip = captures.get(3).unwrap().as_str();
                    
                    alerts_map.insert("ssh_detector".to_string(), format!("Новое SSH-подключение: Пользователь: {}, IP: {}, PID: {}", user, ip, pid));
                }
                
            }

            if alerts_map.keys().len() != 0 {
                let mut response_map = HashMap::new();
                response_map.insert("ssh_detector".to_string(), alerts_map);
                tx.send(response_map).unwrap();
            }
        }
    }
}

pub fn journalctl_watcher(config: Arc<AppConfig>, tx: mpsc::Sender<HashMap<String, HashMap<String, String>>>) {
    
    loop {
        let temp_cooldown = config.ssh_detector.journalctl_cooldown;
        let random_seconds = rand::random_range(0..temp_cooldown);
        std::thread::sleep(std::time::Duration::from_secs(random_seconds));
        let since_arg = format!("{} seconds ago", random_seconds);
        match Command::new("journalctl")
            .arg("-u")
            .arg("ssh") 
            .arg("--since")
            .arg(since_arg)
            .arg("|")
            .arg("grep")
            .arg("Accepted")
            .output()
        {
            Ok(output) if output.status.success() => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let mut alerts_map = HashMap::new();
                for line in stdout.lines() {
                    if line.contains("Accepted") {
                        let parts: Vec<&str> = line.split_whitespace().collect();
    
                        let pid = parts.iter()
                            .find(|&&part| part.starts_with("sshd["))
                            .and_then(|part| part.strip_prefix("sshd[").and_then(|s| s.strip_suffix("]")))
                            .unwrap_or(&"unknown");
    
                        let user = parts.iter()
                            .skip_while(|&&part| part != "for")
                            .nth(1)
                            .unwrap_or(&"unknown");
    
                        // Извлекаем IP (слово после "from")
                        let ip = parts.iter()
                            .skip_while(|&&part| part != "from")
                            .nth(1)
                            .unwrap_or(&"unknown");
    
                        // Формируем строку в требуемом формате
                        let alert = format!("Новое SSH-подключение: Пользователь: {}, IP: {}, PID: {}", user, ip, pid);
                        alerts_map.insert("ssh_detector".to_string(), alert);
                    }
                }
                if !alerts_map.is_empty() {
                    let mut response_map = HashMap::new();
                    response_map.insert("ssh_detector".to_string(), alerts_map);
                    tx.send(response_map).unwrap();
                    continue;
                }
            }
            Ok(output) => {
                let stderr = String::from_utf8_lossy(&output.stderr);
                eprintln!("Error running journalctl: {}", stderr);
            }
            Err(e) => {
                eprintln!("Failed to execute journalctl: {}", e);
            }
        }
    }
}