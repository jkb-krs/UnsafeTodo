use crate::command::Command;
use crate::commands::{add::AddCommand, list::ListCommand, remove::RemoveCommand};

pub fn parse_command(args: &[String]) -> Option<Box<dyn Command>> {
    match args.get(0).map(|s| s.as_str()) {
        Some("--add") => Some(Box::new(AddCommand)),
        Some("--list") => Some(Box::new(ListCommand)),
        Some("--remove") => Some(Box::new(RemoveCommand)),
        _ => None,
    }
}
