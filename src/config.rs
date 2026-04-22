use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use dirs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub default_event: Option<String>,
    pub custom_events: Vec<CustomEvent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomEvent {
    pub name: String,
    pub datetime: String,
    pub timezone: Option<String>,
    pub recurrence: Option<String>,
    pub category: Option<String>,
}

pub fn export_events(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(config) = load_config() {
        let json = serde_json::to_string_pretty(&config.custom_events)?;
        fs::write(path, json)?;
        Ok(())
    } else {
        Err("No config file found".into())
    }
}

pub fn import_events(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let json = fs::read_to_string(path)?;
    let imported_events: Vec<CustomEvent> = serde_json::from_str(&json)?;
    
    let config_path = get_config_path();
    let mut config = if config_path.exists() {
        let config_content = fs::read_to_string(&config_path)?;
        toml::from_str(&config_content)?
    } else {
        Config {
            default_event: None,
            custom_events: Vec::new(),
        }
    };
    
    // Merge imported events with existing events
    config.custom_events.extend(imported_events);
    
    // Save the updated config
    let toml = toml::to_string(&config)?;
    fs::write(&config_path, toml)?;
    
    Ok(())
}

pub fn get_events_by_category(category: &str) -> Vec<crate::events::Event> {
    let mut events = Vec::new();
    if let Some(config) = load_config() {
        for custom_event in config.custom_events {
            if custom_event.category.as_deref() == Some(category) {
                if let Some(timestamp) = crate::time_utils::calculate_last_occurrence(&custom_event.datetime, custom_event.timezone.as_deref(), custom_event.recurrence.as_deref()) {
                    events.push(crate::events::Event {
                        name: custom_event.name,
                        timestamp,
                    });
                }
            }
        }
    }
    events
}

pub fn get_all_categories() -> Vec<String> {
    let mut categories = Vec::new();
    if let Some(config) = load_config() {
        for custom_event in config.custom_events {
            if let Some(category) = &custom_event.category {
                if !categories.contains(category) {
                    categories.push(category.clone());
                }
            }
        }
    }
    categories
}

pub fn get_config_path() -> PathBuf {
    let mut path = dirs::home_dir().expect("Unable to find home directory");
    path.push(".config");
    path.push("since");
    path.push("config.toml");
    path
}

pub fn load_config() -> Option<Config> {
    let config_path = get_config_path();
    if config_path.exists() {
        let config_content = fs::read_to_string(config_path).ok()?;
        toml::from_str(&config_content).ok()
    } else {
        None
    }
}