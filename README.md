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
[2024-03-04T08:22:54.649610+00:00]"create" event detected on path "/Users/axjns/Library/Application Support/dev.warp.Warp-Stable/warp.sqlite-journal"
```

_Post request to remote receiver_
```
POST / HTTP/1.1
accept: */*
host: localhost:8080
content-length: 115

"remove" event detected on path "/Users/axjns/Library/Application Support/dev.warp.Warp-Stable/warp.sqlite-journal"
```