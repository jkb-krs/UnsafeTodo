use crate::command::Command;
use crate::todo::TodoList;

pub struct RemoveCommand;

impl Command for RemoveCommand {
    fn execute(&self, args: &[String], todo_list: &mut TodoList) {
        if let Some(index_str) = args.get(0) {
            if let Ok(index) = index_str.parse::<usize>() {
                if let Some(removed) = todo_list.remove(index - 1) {
                    println!("removed TODO: {}", removed);
                } else {
                    println!("Error: Wrong Index.");
                }
            } else {
                println!("Error: '{}' no valid number", index_str);
            }
        } else {
            println!("Error: No index specified.");
        }
    }
}
