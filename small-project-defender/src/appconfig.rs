use config::{Config, File, Environment};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    hash_scaner: HashScanerConfig,
    telegram_bot: BotConfig,
}

#[derive(Debug,Deserialize)]
pub struct HashScanerConfig {
    directories: Vec<String>,
    exceptions: Vec<String>,
    cooldown: u64,
}

#[derive(Debug,Deserialize)]
pub struct BotConfig {
    token: String,
    admin_chat: String,
}

