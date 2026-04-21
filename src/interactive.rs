use dialoguer::Select;

pub fn interactive_mode() -> crate::events::Event {
    let events = crate::events::get_all_events();
    let event_names: Vec<String> = events.iter().map(|e| e.name.clone()).collect();
    let selection = Select::new()
        .with_prompt("Select an event")
        .items(&event_names)
        .interact()
        .unwrap();
    events[selection].clone()
}