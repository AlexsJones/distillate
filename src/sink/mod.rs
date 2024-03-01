use crate::config::Configuration;
use std::{fs::OpenOptions, io::Write};
use chrono;
pub struct Sink {
    _config: Configuration,
    file: std::fs::File,
}

impl Sink {
    pub fn new(config: Configuration) -> Self {

        // Eventually this will configure all the sinks, for now it's just a log
        let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(config.sink.log_path.clone())
        .unwrap();

        Sink {
            _config: config,
            file: file,
        }
    }
    pub fn emit(&self, message: String) {
        // Write message to self.config.sink.log_path
       
        if let Err(e) = writeln!(&self.file, "[{:?}]{}",chrono::offset::Local::now(), message) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
}