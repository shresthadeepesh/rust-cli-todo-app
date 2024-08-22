use cli_table::{print_stdout, Table, WithTitle};
use core::fmt;
use std::io;

#[derive(Debug)]
pub enum Status {
    Draft,
    Completed,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Table)]
pub struct Todo {
    #[table(title = "ID")]
    pub id: usize,
    #[table(title = "Title")]
    pub title: String,
    #[table(title = "Status")]
    pub status: Status,
}

pub struct TodoApp {
    pub todos: Vec<Todo>,
}

impl TodoApp {
    pub fn insert_todo(&mut self) {
        println!("Enter the todo list: ");

        let mut todo_input = String::new();
        io::stdin()
            .read_line(&mut todo_input)
            .expect("Failed to read line.");

        let todo = Todo {
            id: self.todos.len() + 1,
            title: todo_input,
            status: Status::Draft,
        };

        self.todos.push(todo);

        println!("The todo has been added to the list.");
    }

    pub fn update_todo(&mut self) {
        println!("Enter the id of the todo to update: ");
        let mut todo_input = String::new();
        io::stdin()
            .read_line(&mut todo_input)
            .expect("Failed to read line.");

        let todo_input: usize = match todo_input.trim().parse() {
            Ok(num) => num,
            Err(_) => 0,
        };

        if let Some(item) = self.todos.iter_mut().find(|item| item.id == todo_input) {
            match item.status {
                Status::Completed => {
                    item.status = Status::Draft;
                }
                Status::Draft => {
                    item.status = Status::Completed;
                }
            }
        }

        println!(
            "The selected todo's {} status has been updated.",
            todo_input
        );
    }

    pub fn delete_todo(&mut self) {
        println!("Enter the id of the todo to remove: ");
        let mut todo_input = String::new();
        io::stdin()
            .read_line(&mut todo_input)
            .expect("Failed to read line.");

        let todo_input: usize = match todo_input.trim().parse() {
            Ok(num) => num,
            Err(_) => 0,
        };

        if let Some(pos) = self.todos.iter().position(|item| item.id == todo_input) {
            self.todos.remove(pos);
        }

        println!(
            "The selected todo's {} status has been deleted.",
            todo_input
        );
    }

    pub fn list_todos(&self) {
        assert!(print_stdout(self.todos.with_title()).is_ok());
    }
}
