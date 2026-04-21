use chrono::{DateTime, Utc};
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

#[test]
fn test_parse_datetime_with_timezone() {
    let datetime = "2023-01-01T00:00:00Z";
    let timezone = Some("America/New_York");
    let result = parse_datetime_with_timezone(datetime, timezone);
    assert!(result.is_some());
}

#[test]
fn test_parse_datetime_with_timezone_invalid() {
    let datetime = "invalid";
    let timezone = Some("America/New_York");
    let result = parse_datetime_with_timezone(datetime, timezone);
    assert!(result.is_none());
}

#[test]
fn test_parse_datetime_with_timezone_none() {
    let datetime = "2023-01-01T00:00:00Z";
    let timezone = None;
    let result = parse_datetime_with_timezone(datetime, timezone);
    assert!(result.is_some());
}