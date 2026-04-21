use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use dirs;

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    default_event: Option<String>,
    custom_events: Vec<CustomEvent>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CustomEvent {
    name: String,
    datetime: String,
    timezone: Option<String>,
    recurrence: Option<String>,
    category: Option<String>,
}

fn get_config_path() -> PathBuf {
    let mut path = dirs::home_dir().expect("Unable to find home directory");
    path.push(".config");
    path.push("since");
    path.push("config.toml");
    path
}

fn load_config() -> Option<Config> {
    let config_path = get_config_path();
    if config_path.exists() {
        let config_content = fs::read_to_string(config_path).ok()?;
        toml::from_str(&config_content).ok()
    } else {
        None
    }
}

#[test]
fn test_load_config() {
    let config_path = get_config_path();
    let config_dir = config_path.parent().unwrap();
    
    // Create the config directory if it doesn't exist
    if !config_dir.exists() {
        fs::create_dir_all(config_dir).unwrap();
    }
    
    // Write a test config file
    let config_content = r#"
        default_event = "Last man on the moon"
        
        [[custom_events]]
        name = "Test Event"
        datetime = "2023-01-01T00:00:00Z"
        timezone = "UTC"
        recurrence = "daily"
        category = "test"
    "#;
    
    fs::write(&config_path, config_content).unwrap();
    
    // Load the config
    let config = load_config();
    assert!(config.is_some());
    let config = config.unwrap();
    assert_eq!(config.default_event, Some("Last man on the moon".to_string()));
    assert_eq!(config.custom_events.len(), 1);
    
    // Clean up
    fs::remove_file(config_path).unwrap();
}

#[test]
fn test_load_config_nonexistent() {
    // This test may fail if a config file already exists
    // For the purpose of this test, we'll skip it if the config exists
    let config_path = get_config_path();
    if config_path.exists() {
        return;
    }
    
    let config = load_config();
    assert!(config.is_none());
}