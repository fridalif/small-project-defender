use inotify::{Inotify, WatchMask};
use regex::Regex;
use crate::prelude::*;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};
use std::sync::{Arc, mpsc};
use std::collections::HashMap;

pub fn ssh_auth_log_watcher(config: Arc<AppConfig>, tx: mpsc::Sender<HashMap<String, HashMap<String, String>>>) {
    let mut inotify = Inotify::init().unwrap();

    inotify.watches().add(
        &config.ssh_detector.log_file,
        WatchMask::MODIFY,
    ).unwrap();

    let mut buffer = [0,1024];
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