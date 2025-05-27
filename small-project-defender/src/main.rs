mod appconfig;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let config = appconfig::AppConfig::new("config.yaml".to_string())?;
    print!("{:?}", config);
    Ok(())
}




