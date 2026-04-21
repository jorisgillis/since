use chrono::{DateTime, Utc, Duration, Datelike};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct CustomEvent {
    name: String,
    datetime: String,
    timezone: Option<String>,
    recurrence: Option<String>,
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

fn calculate_last_occurrence(datetime: &str, timezone: Option<&str>, recurrence: Option<&str>) -> Option<DateTime<Utc>> {
    let base_datetime = parse_datetime_with_timezone(datetime, timezone)?;
    
    if let Some(recurrence) = recurrence {
        let now = Utc::now();
        
        match recurrence {
            "daily" => {
                let days_since = (now - base_datetime).num_days();
                let last_day = days_since - (days_since % 1);
                Some(base_datetime + Duration::days(last_day))
            }
            "weekly" => {
                let weeks_since = (now - base_datetime).num_weeks();
                let last_week = weeks_since - (weeks_since % 1);
                Some(base_datetime + Duration::weeks(last_week))
            }
            "monthly" => {
                let mut current = base_datetime;
                while current + Duration::days(30) <= now {
                    current += Duration::days(30);
                }
                Some(current)
            }
            "yearly" => {
                let mut current = base_datetime;
                while current + Duration::days(365) <= now {
                    current += Duration::days(365);
                }
                Some(current)
            }
            _ => None,
        }
    } else {
        Some(base_datetime)
    }
}

#[test]
fn test_daily_recurrence() {
    let custom_event = CustomEvent {
        name: "Daily Event".to_string(),
        datetime: "2023-01-01T00:00:00Z".to_string(),
        timezone: None,
        recurrence: Some("daily".to_string()),
    };
    
    let last_occurrence = calculate_last_occurrence(
        &custom_event.datetime,
        custom_event.timezone.as_deref(),
        custom_event.recurrence.as_deref(),
    );
    
    assert!(last_occurrence.is_some());
    let last_occurrence = last_occurrence.unwrap();
    let now = Utc::now();
    let days_since = (now - last_occurrence).num_days();
    assert!(days_since >= 0 && days_since < 1);
}

#[test]
fn test_weekly_recurrence() {
    let custom_event = CustomEvent {
        name: "Weekly Event".to_string(),
        datetime: "2023-01-01T00:00:00Z".to_string(),
        timezone: None,
        recurrence: Some("weekly".to_string()),
    };
    
    let last_occurrence = calculate_last_occurrence(
        &custom_event.datetime,
        custom_event.timezone.as_deref(),
        custom_event.recurrence.as_deref(),
    );
    
    assert!(last_occurrence.is_some());
    let last_occurrence = last_occurrence.unwrap();
    let now = Utc::now();
    let weeks_since = (now - last_occurrence).num_weeks();
    assert!(weeks_since >= 0 && weeks_since < 1);
}

#[test]
fn test_monthly_recurrence() {
    let custom_event = CustomEvent {
        name: "Monthly Event".to_string(),
        datetime: "2023-01-01T00:00:00Z".to_string(),
        timezone: None,
        recurrence: Some("monthly".to_string()),
    };
    
    let last_occurrence = calculate_last_occurrence(
        &custom_event.datetime,
        custom_event.timezone.as_deref(),
        custom_event.recurrence.as_deref(),
    );
    
    assert!(last_occurrence.is_some());
    let last_occurrence = last_occurrence.unwrap();
    let now = Utc::now();
    let months_since = (now.year() - last_occurrence.year()) * 12 + (now.month() as i32 - last_occurrence.month() as i32);
    assert!(months_since >= 0 && months_since < 1);
}

#[test]
fn test_yearly_recurrence() {
    let custom_event = CustomEvent {
        name: "Yearly Event".to_string(),
        datetime: "2023-01-01T00:00:00Z".to_string(),
        timezone: None,
        recurrence: Some("yearly".to_string()),
    };
    
    let last_occurrence = calculate_last_occurrence(
        &custom_event.datetime,
        custom_event.timezone.as_deref(),
        custom_event.recurrence.as_deref(),
    );
    
    assert!(last_occurrence.is_some());
    let last_occurrence = last_occurrence.unwrap();
    let now = Utc::now();
    let years_since = now.year() - last_occurrence.year();
    println!("Last occurrence: {:?}, Now: {:?}, Years since: {}", last_occurrence, now, years_since);
    assert!(years_since >= 0 && years_since <= 1);
}

#[test]
fn test_no_recurrence() {
    let custom_event = CustomEvent {
        name: "One-time Event".to_string(),
        datetime: "2023-01-01T00:00:00Z".to_string(),
        timezone: None,
        recurrence: None,
    };
    
    let last_occurrence = calculate_last_occurrence(
        &custom_event.datetime,
        custom_event.timezone.as_deref(),
        custom_event.recurrence.as_deref(),
    );
    
    assert!(last_occurrence.is_some());
    let last_occurrence = last_occurrence.unwrap();
    assert_eq!(last_occurrence, DateTime::parse_from_rfc3339("2023-01-01T00:00:00Z").unwrap().with_timezone(&Utc));
}
