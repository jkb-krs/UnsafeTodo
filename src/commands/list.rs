use crate::command::Command;
use crate::todo::TodoList;

pub struct ListCommand;

impl Command for ListCommand {
    fn execute(&self, _args: &[String], todo_list: &mut TodoList) {
        if todo_list.tasks.is_empty() {
            println!("No TODOs found.");
        } else {
            for (i, task) in todo_list.tasks.iter().enumerate() {
                println!("{}: {}", i + 1, task);
            }
        }
    }
}
