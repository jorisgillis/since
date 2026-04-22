use chrono::{DateTime, Utc, Duration};
use chrono_tz::Tz;

pub fn calculate_elapsed_time(event: &crate::events::Event) -> Duration {
    Utc::now() - event.timestamp
}

pub fn calculate_time_until(datetime: &str, timezone: Option<&str>) -> Option<Duration> {
    let future_datetime = parse_datetime_with_timezone(datetime, timezone)?;
    let now = Utc::now();
    
    if future_datetime > now {
        Some(future_datetime - now)
    } else {
        None
    }
}

pub fn parse_datetime_with_timezone(datetime: &str, timezone: Option<&str>) -> Option<DateTime<Utc>> {
    if let Some(tz_str) = timezone {
        if let Ok(tz) = tz_str.parse::<Tz>() {
            // Try to parse the datetime as RFC 3339
            if let Ok(dt) = DateTime::parse_from_rfc3339(datetime) {
                return Some(dt.with_timezone(&tz).with_timezone(&Utc));
            } else {
                // If parsing fails, try to append a "+00:00" to handle datetimes without timezone offsets
                let datetime_with_z = if datetime.ends_with('Z') {
                    datetime.to_string()
                } else {
                    format!("{}+00:00", datetime)
                };
                if let Ok(dt) = DateTime::parse_from_rfc3339(&datetime_with_z) {
                    return Some(dt.with_timezone(&tz).with_timezone(&Utc));
                }
            }
        }
    }
    // Try to parse the datetime as RFC 3339 without timezone
    if let Ok(dt) = DateTime::parse_from_rfc3339(datetime) {
        return Some(dt.with_timezone(&Utc));
    } else {
        // If parsing fails, try to append a "+00:00" to handle datetimes without timezone offsets
        let datetime_with_z = if datetime.ends_with('Z') {
            datetime.to_string()
        } else {
            format!("{}+00:00", datetime)
        };
        DateTime::parse_from_rfc3339(&datetime_with_z).ok()?.with_timezone(&Utc).into()
    }
}

pub fn calculate_last_occurrence(datetime: &str, timezone: Option<&str>, recurrence: Option<&str>) -> Option<DateTime<Utc>> {
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