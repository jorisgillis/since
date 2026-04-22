# since

## Status
**Current version**: V3
**Stage**: Planning

## Shipped
<!-- Items that have been released, oldest first -->
- Support for predefined events (V1)
- Support for custom events via command-line arguments (V1)
- ASCII art output for visual appeal (V1)
- Basic time calculation (years, months, days, hours, minutes, seconds) (V1)
- Add more predefined events (V2)
- Implement random event selection when no event is specified (V2)
- Add support for a configuration file (~/.config/since/config.toml) (V2)
- Allow users to define custom events in the configuration file (V2)
- Categories for organizing events (V3)

## In Development
<!-- Items being actively built right now -->
- Interactive mode for selecting events (V3)
- Support for JSON and plain text output formats (V3)
- Multiple ASCII art themes (V3)
- Time zone support for events (V3)
- Support for recurring events (V3)
- Countdown mode for future events (V3)
- Export and import functionality for custom events (V3)
- Search functionality for events (V3)
- List all events in a table format (V3)
- Generate example config.toml file (V3)
- Default behavior: print default event when no arguments are provided (V3)

## Planned
<!-- Agreed for a future version but not yet started -->

## Backlog
<!-- Ideas and requests not yet assigned to a version -->
- Advanced time calculations (e.g., business days, time zones)
- Interactive mode for selecting events
- Multiple output formats (e.g., JSON, plain text)

## Key Decisions
| Date       | Decision                          | Rationale                          |
|------------|-----------------------------------|------------------------------------|
| 2024-04-21 | Use ASCII art for output          | To match the playful nature of the tool |
| 2024-04-21 | Support both predefined and custom events | To provide flexibility for users |
| 2024-04-21 | Add configuration file support     | To allow users to customize their experience |