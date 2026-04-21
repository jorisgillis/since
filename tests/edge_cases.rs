use chrono::{DateTime, Utc, Duration};

#[derive(Clone)]
struct Event {
    name: String,
    timestamp: DateTime<Utc>,
}

fn calculate_elapsed_time(event: &Event) -> Duration {
    Utc::now() - event.timestamp
}

fn calculate_time_until(datetime: &str, timezone: Option<&str>) -> Option<Duration> {
    let future_datetime = parse_datetime_with_timezone(datetime, timezone)?;
    let now = Utc::now();
    
    if future_datetime > now {
        Some(future_datetime - now)
    } else {
        None
    }
}

fn parse_datetime_with_timezone(datetime: &str, timezone: Option<&str>) -> Option<DateTime<Utc>> {
    if let Some(tz_str) = timezone {
        if let Ok(tz) = tz_str.parse::<chrono_tz::Tz>() {
            if let Ok(dt) = DateTime::parse_from_rfc3339(datetime) {
                return Some(dt.with_timezone(&tz).with_timezone(&Utc));
            }
        }
    }
    DateTime::parse_from_rfc3339(datetime).ok()?.with_timezone(&Utc).into()
}

#[test]
fn test_calculate_elapsed_time() {
    let event = Event {
        name: "Test Event".to_string(),
        timestamp: chrono::Utc::now() - chrono::Duration::days(1),
    };
    let duration = calculate_elapsed_time(&event);
    assert!(duration.num_days() >= 1);
}

#[test]
fn test_calculate_time_until() {
    let datetime = (chrono::Utc::now() + chrono::Duration::days(1)).to_rfc3339();
    let timezone = None;
    let result = calculate_time_until(&datetime, timezone);
    assert!(result.is_some());
}

#[test]
fn test_calculate_time_until_past() {
    let datetime = (chrono::Utc::now() - chrono::Duration::days(1)).to_rfc3339();
    let timezone = None;
    let result = calculate_time_until(&datetime, timezone);
    assert!(result.is_none());
}