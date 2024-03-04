use crate::config::Configuration;
use chrono;
use reqwest;
use std::{fs::OpenOptions, io::Write, time::Duration};
pub struct Sink {
    config: Configuration,
    file: std::fs::File,
    http_client: reqwest::Client,
    default_timeout: u64
}

impl Sink {
    pub fn new(config: Configuration) -> Self {
        // if config.sink.log_path is empty, default to local directory
        let mut log_path = "distillate.log";
        if !config.sink.log_path.is_empty() {
            log_path = config.sink.log_path.as_str();
        }
        let client = reqwest::Client::new();
        // Eventually this will configure all the sinks, for now it's just a log
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(log_path)
            .unwrap();

        Sink {
            config: config,
            file: file,
            http_client: client,    
            default_timeout: Duration::from_secs(10).as_secs()
        }
    }
    pub async fn emit(&self, message: String) -> Result<(),Box<dyn std::error::Error>> {
        // Write message to self.config.sink.log_path

            if let Err(e) = writeln!(
                &self.file,
                "[{:?}]{}",
                chrono::offset::Local::now(),
                message
            ) {
                Err(Box::new(e))?;
            }
    
        if !self.config.sink.webhook.url.is_empty() {
            self.http_client
                .post(&self.config.sink.webhook.url)
                .body(message)
                .timeout(Duration::from_secs(self.default_timeout))
                .send().await?;
        }

        Ok(())
    }
}
