# Distillate

Distillate will watch filesystem resources and notify on changes to selected resources.
The typical usecase would be to monitor for tampering of critical system files.

## Installation
```
cargo install distillate
```

## Usage

```
distillate run --options-path=distillate.config
```

Example configuration

```
{
    "fuzzy_paths": true,
    "watch_paths": [{
    "path": "/etc",
    "recursive": true,
    "alert_on": [
        {
            "event_type": "create",
            "path": "/etc/hosts"
        }
    ]
    }],
    "sink": {
        "log_path": "monitoring.log"
    }
} 
```
Example monitoring output 
```
[2024-03-01T11:17:00.053856+00:00]"create" event detected on path "/private/etc/1"
[2024-03-01T11:17:01.271469+00:00]"create" event detected on path "/private/etc/1"
[2024-03-01T11:17:01.271617+00:00]"remove" event detected on path "/private/etc/1"
```