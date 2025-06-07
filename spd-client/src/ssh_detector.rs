use inotify::{Inotify, WatchMask};
use regex::Regex;
use crate::prelude::*;

fn ssh_detector(config: Arc<AppConfig>, tx: mpsc::Sender<HashMap<String, String>>) {
    let mut inotify = Inotify::init()?;

    inotify.watches().add(
        &config.ssh_detector.path,
        WatchMask::MODIFY,
    )?;

    let mut buffer = [0,1024];
    let re = Regex::new(r"sshd\[(\d+)\].*Accepted.*for (\w+) from ([\d\.]+)").unwrap();

    loop {
        let events = inotify.read_events(&mut buffer).unwrap();
    }
}