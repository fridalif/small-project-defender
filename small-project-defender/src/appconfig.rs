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

impl AppConfig {
    pub fn new(path: String) -> Result<Self, Box<dyn std::error::Error>> {
        let yaml_str =  std::fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read YAML file '{}': {}", path, e))?;

        // Десериализуем YAML в структуру AppConfig
        let config: AppConfig = serde_yaml::from_str(&yaml_str)
            .map_err(|e| format!("Failed to parse YAML: {}", e))?;

        Ok(config)
    }
}