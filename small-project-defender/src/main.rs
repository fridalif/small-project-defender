mod appconfig;
mod hash_scaner;
mod prelude {
    pub use crate::appconfig::AppConfig;
}

use crate::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let config = std::sync::Arc::new(AppConfig::new("../etc/config.yaml".to_string())?);
    let origins_hash_scaner = hash_scaner::init_origins_hash_scaner(std::sync::Arc::clone(&config));
    hash_scaner::schedule_hash_scaner(origins_hash_scaner, std::sync::Arc::clone(&config));
    Ok(())
}




