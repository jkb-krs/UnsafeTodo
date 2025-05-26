use crate::command::Command;
use crate::todo::TodoList;

pub struct RemoveCommand;

impl Command for RemoveCommand {
    fn execute(&self, args: &[String], todo_list: &mut TodoList) {
        if let Some(index_str) = args.get(0) {
            if let Ok(index) = index_str.parse::<usize>() {
                if let Some(removed) = todo_list.remove(index - 1) {
                    println!("Aufgabe entfernt: {}", removed);
                } else {
                    println!("Fehler: Ungültiger Index.");
                }
            } else {
                println!("Fehler: '{}' ist keine gültige Zahl.", index_str);
            }
        } else {
            println!("Fehler: Kein Index angegeben.");
        }
    }
}
