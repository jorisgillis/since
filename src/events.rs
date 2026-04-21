use chrono::{DateTime, Utc};
use rand::prelude::SliceRandom;

#[derive(Clone)]
pub struct Event {
    pub name: String,
    pub timestamp: DateTime<Utc>,
}

pub fn get_predefined_events() -> Vec<Event> {
    vec![
        Event {
            name: "Last man on the moon".to_string(),
            timestamp: DateTime::parse_from_rfc3339("1972-12-14T05:40:00Z")
                .unwrap()
                .with_timezone(&Utc),
        },
        Event {
            name: "First man on the moon".to_string(),
            timestamp: DateTime::parse_from_rfc3339("1969-07-20T20:17:00Z")
                .unwrap()
                .with_timezone(&Utc),
        },
        Event {
            name: "Birth of the internet".to_string(),
            timestamp: DateTime::parse_from_rfc3339("1983-01-01T00:00:00Z")
                .unwrap()
                .with_timezone(&Utc),
        },
        Event {
            name: "Fall of the Berlin Wall".to_string(),
            timestamp: DateTime::parse_from_rfc3339("1989-11-09T00:00:00Z")
                .unwrap()
                .with_timezone(&Utc),
        },
    ]
}

pub fn get_predefined_event(event_name: &str) -> Option<Event> {
    let predefined_events = get_predefined_events();
    predefined_events
        .into_iter()
        .find(|e| e.name.to_lowercase() == event_name.to_lowercase())
}

pub fn get_random_predefined_event() -> Event {
    let predefined_events = get_predefined_events();
    let mut rng = rand::thread_rng();
    predefined_events.choose(&mut rng).unwrap().clone()
}

pub fn search_events(query: &str) -> Vec<Event> {
    let all_events = get_all_events();
    all_events
        .into_iter()
        .filter(|event| event.name.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}



pub fn get_all_events() -> Vec<Event> {
    let mut events = get_predefined_events();
    if let Some(config) = crate::config::load_config() {
        for custom_event in config.custom_events {
            if let Some(timestamp) = crate::time_utils::calculate_last_occurrence(&custom_event.datetime, custom_event.timezone.as_deref(), custom_event.recurrence.as_deref()) {
                events.push(Event {
                    name: custom_event.name,
                    timestamp,
                });
            }
        }
    }
    events
}

pub fn get_custom_event_from_config(event_name: &str) -> Option<Event> {
    if let Some(config) = crate::config::load_config() {
        for custom_event in config.custom_events {
            if custom_event.name.to_lowercase() == event_name.to_lowercase() {
                return Some(Event {
                    name: custom_event.name,
                    timestamp: crate::time_utils::calculate_last_occurrence(&custom_event.datetime, custom_event.timezone.as_deref(), custom_event.recurrence.as_deref())?,
                });
            }
        }
    }
    None
}