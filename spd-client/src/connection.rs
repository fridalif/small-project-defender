use crate::prelude::*;
use std::sync::{mpsc, Arc, Mutex};
use std::collections::HashMap;

pub fn serve(config: Arc<AppConfig>, rx: mpsc::Receiver<HashMap<String, HashMap<String, String>>>, hash_origins: Arc<Mutex<HashMap<String, String>>>) {
    return    
}