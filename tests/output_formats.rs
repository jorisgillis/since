use chrono::{DateTime, Utc};
use colored::*;

#[derive(Clone)]
struct Event {
    name: String,
    timestamp: DateTime<Utc>,
}

fn generate_ascii_art(text: &str) -> String {
    let border = "╔".to_owned() + &"═".repeat(text.len() + 2) + "╗";
    let empty_line = "║".to_owned() + &" ".repeat(text.len() + 2) + "║";
    let content_line = format!("║ {} ║", text);
    let footer = "╚".to_owned() + &"═".repeat(text.len() + 2) + "╝";

    format!(
        "{}\n{}\n{}\n{}\n{}",
        border, empty_line, content_line, empty_line, footer
    )
}

fn generate_fancy_ascii_art(text: &str) -> String {
    let border = "╔".to_owned() + &"═".repeat(text.len() + 2) + "╗";
    let empty_line = "║".to_owned() + &" ".repeat(text.len() + 2) + "║";
    let content_line = format!("║ {} ║", text);
    let footer = "╚".to_owned() + &"═".repeat(text.len() + 2) + "╝";

    format!(
        "{}\n{}\n{}\n{}\n{}",
        border, empty_line, content_line, empty_line, footer
    )
}

fn generate_colored_ascii_art(text: &str) -> String {
    let border = format!("{}{}{}", "╔".green().bold(), "═".repeat(text.len() + 2).green().bold(), "╗".green().bold());
    let empty_line = format!("{}{}{}", "║".green().bold(), " ".repeat(text.len() + 2).green().bold(), "║".green().bold());
    let content_line = format!("║ {} ║", text.yellow().bold());
    let footer = format!("{}{}{}", "╚".green().bold(), "═".repeat(text.len() + 2).green().bold(), "╝".green().bold());

    format!(
        "{}\n{}\n{}\n{}\n{}",
        border, empty_line, content_line, empty_line, footer
    )
}

#[test]
fn test_generate_ascii_art() {
    let text = "Test Message";
    let ascii_art = generate_ascii_art(text);
    assert!(ascii_art.contains(text));
}

#[test]
fn test_generate_fancy_ascii_art() {
    let text = "Test Message";
    let ascii_art = generate_fancy_ascii_art(text);
    assert!(ascii_art.contains(text));
}

#[test]
fn test_generate_colored_ascii_art() {
    let text = "Test Message";
    let ascii_art = generate_colored_ascii_art(text);
    assert!(ascii_art.contains(text));
}