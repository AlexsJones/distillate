use crate::config;
use log::{info, debug};

pub struct Processor{
    configuration: config::Configuration,
}

impl Processor {
    pub fn new(conf: config::Configuration) -> Self {
        Processor{
            configuration: conf,
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
    pub fn process_event(&self, event: notify::Event) {
        debug!("event: {:?}", event);

        // filter based on the event path 
        let path = event.paths.get(0).unwrap();
        let path_str = path.to_str().unwrap();
        
        let mut path_watch: Option<&config::PathWatch> = None;
        if self.configuration.fuzzy_paths {
            path_watch = self.configuration.watch_paths.iter().find(|p| path_str.contains(&p.path));
        }else {
            path_watch = self.configuration.watch_paths.iter().find(|p| path_str.starts_with(&p.path));
        }
        
        if path_watch.is_none() {
            debug!("path not found: {:?}", path_str);
            return;
        }
        let path_watch = path_watch.unwrap();
        debug!("path_watch: {:?}", path_watch.path);

        for alert in &path_watch.alert_on {
            
            // if the path matches the alert path
            if path_str.contains(&alert.path) {
              
              let event_type = self.map_event_type(&event);
              if alert.event_type == event_type {
                  info!("alert: {:?}", alert.path);
              }    
            

        }
       
    }
    }
}