use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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

fn get_all_events() -> Vec<Event> {
    get_predefined_events()
}

#[test]
fn test_interactive_mode() {
    let events = get_all_events();
    assert!(!events.is_empty());
    
    // Note: This test is limited because interactive_mode requires user input.
    // A full integration test would require mocking user input.
}