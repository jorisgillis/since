use chrono::Duration;
use colored::*;

pub fn format_duration(duration: Duration) -> String {
    let years = duration.num_days() / 365;
    let remaining_days = duration.num_days() % 365;
    let months = remaining_days / 30;
    let days = remaining_days % 30;
    let hours = duration.num_hours() % 24;
    let minutes = duration.num_minutes() % 60;
    let seconds = duration.num_seconds() % 60;

    format!(
        "{}y {}mo {}d {}h {}m {}s",
        years, months, days, hours, minutes, seconds
    )
}

pub fn generate_ascii_art(text: &str) -> String {
    let border = "╔".to_owned() + &"═".repeat(text.len() + 2) + "╗";
    let empty_line = "║".to_owned() + &" ".repeat(text.len() + 2) + "║";
    let content_line = format!("║ {} ║", text);
    let footer = "╚".to_owned() + &"═".repeat(text.len() + 2) + "╝";

    format!(
        "{}\n{}\n{}\n{}\n{}",
        border, empty_line, content_line, empty_line, footer
    )
}

pub fn generate_fancy_ascii_art(text: &str) -> String {
    let border = "╔".to_owned() + &"═".repeat(text.len() + 2) + "╗";
    let empty_line = "║".to_owned() + &" ".repeat(text.len() + 2) + "║";
    let content_line = format!("║ {} ║", text);
    let footer = "╚".to_owned() + &"═".repeat(text.len() + 2) + "╝";

    format!(
        "{}\n{}\n{}\n{}\n{}",
        border, empty_line, content_line, empty_line, footer
    )
}

pub fn generate_colored_ascii_art(text: &str) -> String {
    let border = format!("{}{}{}", "╔".green().bold(), "═".repeat(text.len() + 2).green().bold(), "╗".green().bold());
    let empty_line = format!("{}{}{}", "║".green().bold(), " ".repeat(text.len() + 2).green().bold(), "║".green().bold());
    let content_line = format!("║ {} ║", text.yellow().bold());
    let footer = format!("{}{}{}", "╚".green().bold(), "═".repeat(text.len() + 2).green().bold(), "╝".green().bold());

    format!(
        "{}\n{}\n{}\n{}\n{}",
        border, empty_line, content_line, empty_line, footer
    )
}