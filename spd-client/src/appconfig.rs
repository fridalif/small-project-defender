use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub hash_scaner: HashScanerConfig,
}

#[derive(Debug,Deserialize)]
pub struct HashScanerConfig {
    pub on: bool,
    pub directories: Vec<String>,
    pub exceptions: Vec<String>,
    pub cooldown: u64,
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