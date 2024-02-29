use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize,Clone)]
pub struct AlertOn {
    pub path: String,
    pub event_type: String,
}
#[derive(Serialize, Deserialize,Clone)]
pub struct PathWatch {
    pub path: String,
    pub recursive: bool,
    pub alert_on: Vec<AlertOn>,
}

#[derive(Serialize, Deserialize,Clone)]
pub struct Configuration {
    pub fuzzy_paths: bool,
    pub watch_paths: Vec<PathWatch>,
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
