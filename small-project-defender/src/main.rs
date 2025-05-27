mod appconfig;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let config = appconfig::AppConfig::new("../etc/config.yaml".to_string())?;
    print!("{:?}", config);
    Ok(())
}




