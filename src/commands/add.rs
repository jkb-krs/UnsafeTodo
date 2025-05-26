use crate::command::Command;
use crate::todo::TodoList;

pub struct AddCommand;

impl Command for AddCommand {
    fn execute(&self, args: &[String], todo_list: &mut TodoList) {
        if let Some(task) = args.get(0) {
            todo_list.add(task.to_string());
            println!("Added task: {}", task);
        } else {
            println!("Error: no task specified.");
        }
    }
}
