use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub server_settings: ServerSettings
}

pub struct ServerSettings {
    pub ip: String,
    pub port: u16,
    pub init_secret: String    
}

impl AppConfig {
    pub fn new(path: String) -> Result<Self, Box<dyn std::error::Error>> {
        let yaml_str =  std::fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read YAML file '{}': {}", path, e))?;

        let config: AppConfig = serde_yaml::from_str(&yaml_str)
            .map_err(|e| format!("Failed to parse YAML: {}", e))?;

        Ok(config)
    }
}