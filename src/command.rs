use crate::todo::TodoList;

pub trait Command {
    fn execute(&self, args: &[String], todo_list: &mut TodoList);
}
