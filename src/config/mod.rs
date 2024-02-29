use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    pub watch_paths: Vec<String>,
}

pub fn load_config(path: &str) -> Result<Configuration, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);
    let config = serde_json::from_reader(reader);
    match config {
        Ok(config) => Ok(config),
        Err(e) => Err(Box::new(e)),
    }
}