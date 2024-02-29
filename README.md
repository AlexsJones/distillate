# Distillate

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
    }]
} 
```