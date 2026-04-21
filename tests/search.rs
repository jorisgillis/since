use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct CustomEvent {
    name: String,
    datetime: String,
    timezone: Option<String>,
    recurrence: Option<String>,
    category: Option<String>,
}

#[derive(Clone)]
struct Event {
    name: String,
    timestamp: DateTime<Utc>,
}

fn get_predefined_events() -> Vec<Event> {
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

fn search_events(query: &str) -> Vec<Event> {
    let all_events = get_predefined_events();
    all_events
        .into_iter()
        .filter(|event| event.name.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[test]
fn test_search_events() {
    let events = search_events("moon");
    assert!(!events.is_empty());
    assert!(events.iter().any(|e| e.name.contains("moon")));
}

#[test]
fn test_search_events_no_results() {
    let events = search_events("nonexistent");
    assert!(events.is_empty());
}

#[test]
fn test_search_events_case_insensitive() {
    let events = search_events("MOON");
    assert!(!events.is_empty());
    assert!(events.iter().any(|e| e.name.contains("moon")));
}