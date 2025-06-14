use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub hash_scaner: HashScanerConfig,
    pub ssh_detector: SSHDetectorConfig,
    pub port_detector: PortDetectorConfig
}

#[derive(Debug,Deserialize)]
pub struct HashScanerConfig {
    pub on: bool,
    pub directories: Vec<String>,
    pub exceptions: Vec<String>,
    pub cooldown: u64,
}

#[derive(Debug,Deserialize)]
pub struct SSHDetectorConfig {
    pub check_auth_on: bool,
    pub log_file: String,
    pub check_journalctl_on: bool,
    pub journalctl_cooldown: u64,
}

#[derive(Debug,Deserialize)]
pub struct PortDetectorConfig {
    pub on: bool,
    pub legit_ports: Vec<String>,
    pub cooldown: u64
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