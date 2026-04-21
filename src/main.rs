use clap::{Parser, Subcommand};
use serde_json;

mod events;
mod time_utils;
mod config;
mod output;
mod interactive;

#[derive(Parser)]
#[command(name = "since")]
#[command(about = "A fun CLI tool to show time elapsed since an event", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    /// Output format (ascii, plain, json)
    #[arg(short, long, default_value = "ascii")]
    format: String,
    /// ASCII art theme (simple, fancy, colored)
    #[arg(short, long, default_value = "simple")]
    theme: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Show time since a predefined event
    Predefined {
        /// Name of the predefined event
        event: Option<String>,
    },
    /// Show time since a custom event
    Custom {
        /// Name of the custom event
        name: String,
        /// Date and time of the event (ISO 8601 format)
        datetime: String,
        /// Time zone of the event (e.g., "America/New_York")
        timezone: Option<String>,
    },
    /// Show time until a future event
    Countdown {
        /// Name of the event
        name: String,
        /// Date and time of the event (ISO 8601 format)
        datetime: String,
        /// Time zone of the event (e.g., "America/New_York")
        timezone: Option<String>,
    },
    /// Interactive mode
    Interactive,
    /// Search for events
    Search {
        /// Search query
        query: String,
    },
    /// List events by category
    Category {
        /// Category name (optional, lists all categories if not provided)
        category: Option<String>,
    },
    /// List all events in a table format
    List,
}

impl Commands {
    fn is_countdown(&self) -> bool {
        matches!(self, Commands::Countdown { .. })
    }
}

fn main() {
    let cli = Cli::parse();

    let (event_name, time_remaining) = match &cli.command {
        Commands::Predefined { event } => {
            let event = match event {
                Some(event_name) => {
                    if let Some(e) = events::get_predefined_event(event_name) {
                        e
                    } else if let Some(e) = events::get_custom_event_from_config(event_name) {
                        e
                    } else {
                        eprintln!("Unknown event: {}", event_name);
                        std::process::exit(1);
                    }
                }
                None => {
                    if let Some(config) = config::load_config() {
                        if let Some(default_event) = config.default_event {
                            if let Some(e) = events::get_predefined_event(&default_event) {
                                e
                            } else if let Some(e) = events::get_custom_event_from_config(&default_event) {
                                e
                            } else {
                                events::get_random_predefined_event()
                            }
                        } else {
                            events::get_random_predefined_event()
                        }
                    } else {
                        events::get_random_predefined_event()
                    }
                }
            };
            (event.name.clone(), Some(time_utils::calculate_elapsed_time(&event)))
        }
        Commands::Custom { name, datetime, timezone } => {
            match time_utils::parse_datetime_with_timezone(datetime, timezone.as_deref()) {
                Some(dt) => {
                    let event = events::Event {
                        name: name.clone(),
                        timestamp: dt,
                    };
                    (event.name.clone(), Some(time_utils::calculate_elapsed_time(&event)))
                }
                None => {
                    eprintln!("Invalid datetime format or timezone");
                    std::process::exit(1);
                }
            }
        }
        Commands::Countdown { name, datetime, timezone } => {
            match time_utils::calculate_time_until(datetime, timezone.as_deref()) {
                Some(duration) => (name.clone(), Some(duration)),
                None => {
                    eprintln!("Event is in the past");
                    std::process::exit(1);
                }
            }
        }
        Commands::Interactive => {
            let event = interactive::interactive_mode();
            (event.name.clone(), Some(time_utils::calculate_elapsed_time(&event)))
        }
        Commands::Search { query } => {
            let events = events::search_events(query);
            if events.is_empty() {
                eprintln!("No events found matching: {}", query);
                std::process::exit(1);
            }
            let event = &events[0];
            (event.name.clone(), Some(time_utils::calculate_elapsed_time(event)))
        }
        Commands::Category { category } => {
            match category {
                Some(cat) => {
                    let events = config::get_events_by_category(cat);
                    if events.is_empty() {
                        eprintln!("No events found in category: {}", cat);
                        std::process::exit(1);
                    }
                    let event = &events[0];
                    (event.name.clone(), Some(time_utils::calculate_elapsed_time(event)))
                }
                None => {
                    let categories = config::get_all_categories();
                    if categories.is_empty() {
                        eprintln!("No categories found");
                        std::process::exit(1);
                    }
                    println!("Available categories:");
                    for category in categories {
                        println!("- {}", category);
                    }
                    std::process::exit(0);
                }
            }
        }
        Commands::List => {
            let all_events = events::get_all_events();
            if all_events.is_empty() {
                eprintln!("No events found");
                std::process::exit(1);
            }
            println!("{:<30} {:<20}", "Name", "Date");
            println!("{:<30} {:<20}", "-", "---");
            for event in all_events {
                let date = event.timestamp.format("%Y-%m-%d %H:%M:%S").to_string();
                println!("{:<30} {:<20}", event.name, date);
            }
            std::process::exit(0);
        }
    };

    let time_remaining = time_remaining.expect("Time remaining should be calculated");
    let formatted_time = output::format_duration(time_remaining);
    let message = if cli.command.is_countdown() {
        format!("{} until {}", formatted_time, event_name)
    } else {
        format!("It has been {} since {}", formatted_time, event_name)
    };

    match cli.format.as_str() {
        "ascii" => {
            let ascii_art = match cli.theme.as_str() {
                "simple" => output::generate_ascii_art(&message),
                "fancy" => output::generate_fancy_ascii_art(&message),
                "colored" => output::generate_colored_ascii_art(&message),
                _ => {
                    eprintln!("Unknown theme: {}", cli.theme);
                    std::process::exit(1);
                }
            };
            println!("{}", ascii_art);
        }
        "plain" => {
            println!("{}", message);
        }
        "json" => {
            let json_output = serde_json::json!({
                "event": event_name,
                "time_remaining": formatted_time,
                "is_countdown": cli.command.is_countdown(),
            });
            println!("{}", json_output);
        }
        _ => {
            eprintln!("Unknown output format: {}", cli.format);
            std::process::exit(1);
        }
    }
}