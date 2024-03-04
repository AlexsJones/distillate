use crate::config;
use crate::sink::Sink;
use log::{debug, info};

pub struct Processor {
    configuration: config::Configuration,
    sink: Sink,
}

impl Processor {
    pub fn new(conf: config::Configuration) -> Self {
        Processor {
            configuration: conf.clone(),
            sink: Sink::new(conf.clone()),
        }
    }
    pub fn map_event_type(&self, event: &notify::Event) -> String {
        let event_type = match event.kind {
            notify::EventKind::Create(_) => "create",
            notify::EventKind::Modify(_) => "modify",
            notify::EventKind::Remove(_) => "remove",
            notify::EventKind::Access(_) => "access",
            notify::EventKind::Any => "any",
            notify::EventKind::Other => "other",
        };
        event_type.to_string()
    }
    pub async fn process_event(&self, event: notify::Event) {
        debug!("event: {:?}", event);

        // filter based on the event path
        let path = event.paths.get(0).unwrap();
        let path_str = path.to_str().unwrap();

        let mut path_watch: Option<&config::PathWatch> = None;
        if self.configuration.fuzzy_paths {
            path_watch = self
                .configuration
                .watch_paths
                .iter()
                .find(|p| path_str.contains(&p.path));
        } else {
            path_watch = self
                .configuration
                .watch_paths
                .iter()
                .find(|p| path_str.starts_with(&p.path));
        }

        if path_watch.is_none() {
            debug!("path not found: {:?}", path_str);
            return;
        }
        let path_watch = path_watch.unwrap();
        debug!("path_watch: {:?}", path_watch.path);

        for alert in &path_watch.alert_on {
            // if the path matches the alert path
            if path_str.contains(&alert.path) || alert.path == "*" {
                let event_type = self.map_event_type(&event);
                if alert.event_type == event_type {
                    debug!(
                        "alert detected for path: {:?} and event: {:?}",
                        path_str, event_type
                    );
                    let res = self.sink.emit(format!(
                        "{:?} event detected on path {:?}",
                        event_type, path_str
                    )).await;
                    match res {
                        Ok(_) => {
                            info!("sink sent");
                        }
                        Err(e) => {
                            eprintln!("error sending alert: {:?}", e);
                        }
                    }
                }
            }
        }
    }
}
