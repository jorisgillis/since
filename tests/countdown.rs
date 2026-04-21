use chrono::{DateTime, Utc, Duration};
use chrono_tz::Tz;

fn parse_datetime_with_timezone(datetime: &str, timezone: Option<&str>) -> Option<DateTime<Utc>> {
    if let Some(tz_str) = timezone {
        if let Ok(tz) = tz_str.parse::<Tz>() {
            if let Ok(dt) = DateTime::parse_from_rfc3339(datetime) {
                return Some(dt.with_timezone(&tz).with_timezone(&Utc));
            }
        }
    }
    DateTime::parse_from_rfc3339(datetime).ok()?.with_timezone(&Utc).into()
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

#[test]
fn test_time_until_future_event() {
    let future_datetime = "2026-12-31T23:59:59Z";
    let time_until = calculate_time_until(future_datetime, None);
    
    assert!(time_until.is_some());
    let time_until = time_until.unwrap();
    assert!(time_until > Duration::zero());
}

#[test]
fn test_time_until_past_event() {
    let past_datetime = "2020-01-01T00:00:00Z";
    let time_until = calculate_time_until(past_datetime, None);
    
    assert!(time_until.is_none());
}

#[test]
fn test_time_until_with_timezone() {
    let future_datetime = "2026-12-31T23:59:59-05:00";
    let timezone = "America/New_York";
    let time_until = calculate_time_until(future_datetime, Some(timezone));
    
    assert!(time_until.is_some());
    let time_until = time_until.unwrap();
    assert!(time_until > Duration::zero());
}
