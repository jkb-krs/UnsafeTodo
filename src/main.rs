mod command;
mod commands;
mod parser;
mod todo;

use parser::parse_command;
use todo::TodoList;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let mut todo_list = TodoList::load().unwrap_or_else(|_| TodoList::new());
    
    match parse_command(&args) {
        Some(cmd) => {
            cmd.execute(&args[1..], &mut todo_list);
        }
        None => {
            eprintln!("Unknown or missing command.");
            print_help();
        }
    }
    
    if let Err(e) = todo_list.save() {
        eprintln!("Failed to safe {}", e);
    }

}

fn print_help() {
    println!("Help:");
    println!("  unsafe__todo --add \"Task\"");
    println!("  unsafe__todo --list");
    println!("  unsafe__todo --remove <index>");
}
