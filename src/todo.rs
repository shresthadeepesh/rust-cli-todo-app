use cli_table::{print_stdout, Table, WithTitle};
use core::fmt;
use std::{
    fs::File,
    io::{self, Write},
};

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
    todos: Vec<Todo>,
}

impl TodoApp {
    pub fn new() -> Self {
        TodoApp { todos: Vec::new() }
    }

    pub fn add_todo(&mut self, todo: Todo) {
        self.todos.push(todo);
    }

    pub fn todos_len(&self) -> usize {
        self.todos.len()
    }

    pub fn generate_id(&self) -> usize {
        if let Some(max_id) = self.todos.iter().map(|todo| todo.id).max() {
            max_id + 1
        } else {
            1
        }
    }

    pub fn insert_todo(&mut self) {
        println!("Enter the todo list: ");

        let mut todo_input = String::new();
        io::stdin()
            .read_line(&mut todo_input)
            .expect("Failed to read line.");

        let todo = Todo {
            id: self.generate_id(),
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
            Err(_) => {
                println!("Invalid ID entered.");
                return;
            }
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
            Err(_) => {
                println!("Invalid ID entered.");
                return;
            }
        };

        if let Some(pos) = self.todos.iter().position(|item| item.id == todo_input) {
            self.todos.remove(pos);
        }

        println!("The selected todo's {} has been deleted.", todo_input);
    }

    pub fn list_todos(&self) {
        assert!(print_stdout(self.todos.with_title()).is_ok());
    }

    pub fn seed_todos(&mut self) {
        for i in 1..10 {
            let todo = Todo {
                id: i,
                title: format!("Todo list {}", i),
                status: Status::Draft,
            };

            self.add_todo(todo);
        }
    }

    pub fn export_to_csv(&self) {
        let mut csv_str = String::from("ID,Title,Status\n");

        for todo in &self.todos {
            let todo_str = format!("{},{},{}\n", todo.id, todo.title, todo.status);
            csv_str.push_str(&todo_str);
        }

        println!("Exporting the following list of todos.");
        println!("{}", csv_str);

        let mut file = match File::create("todos.csv") {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Failed to create a file: {}", e);
                return;
            }
        };

        if let Err(e) = file.write_all(csv_str.as_bytes()) {
            eprintln!("Failed to write to file: {}", e);
        }
    }
}
