mod appconfig;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let config = std::sync::Arc::new(appconfig::AppConfig::new("../etc/config.yaml".to_string())?);
    print!("{:?}", config);
    Ok(())
}




