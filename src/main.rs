mod command;
mod commands;
mod parser;
mod todo;

use parser::parse_command;
use todo::TodoList;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let mut todo_list = TodoList::load("todo.json").unwrap_or_else(|_| TodoList::new());

    match parse_command(&args) {
        Some(cmd) => {
            cmd.execute(&args[1..], &mut todo_list);
        }
        None => {
            eprintln!("Unbekannter oder fehlender Befehl.");
            print_help();
        }
    }

    if let Err(e) = todo_list.save("todo.json") {
        eprintln!("Fehler beim Speichern: {}", e);
    }
}

fn print_help() {
    println!("Verwendung:");
    println!("  unsafe_rs --add \"Aufgabe\"");
    println!("  unsafe_rs --list");
    println!("  unsafe_rs --remove <index>");
}
