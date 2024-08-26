use chrono::{DateTime, Utc};
use cli_table::{format::Justify, Cell, Style, Table};
use core::fmt;
use std::{
    fs::File,
    io::{self, Write},
};

#[derive(Debug, Clone)]
pub enum Status {
    Draft,
    Completed,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct Todo {
    pub id: usize,
    pub title: String,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
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
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
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

    pub fn soft_delete_todo(&mut self) {
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

        if let Some(item) = self.todos.iter_mut().find(|item| item.id == todo_input) {
            item.deleted_at = Some(Utc::now());
            println!("The selected todo {} has been soft deleted.", todo_input);
        } else {
            println!("The selected todo {} doesn't exist.", todo_input);
        }
    }

    pub fn restore_todo(&mut self) {
        println!("Enter the id of the todo to restore: ");
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
            item.deleted_at = None;
            println!("The selected todo {} has been restored.", todo_input);
        } else {
            println!("The selected todo {} doesn't exist.", todo_input);
        }
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
            println!("The selected todo {} has been deleted.", todo_input);
        } else {
            println!("The selected todo {} doesn't exist.", todo_input);
        }
    }

    pub fn list_todos(&self) {
        let rows = self
            .todos
            .iter()
            .map(|todo| {
                vec![
                    todo.id.cell(),
                    todo.title.clone().cell(),
                    todo.status.clone().cell(),
                    todo.created_at.cell(),
                    todo.updated_at.cell(),
                    match todo.deleted_at {
                        Some(val) => val.cell(),
                        None => "".cell(),
                    },
                ]
            })
            .table()
            .title(vec![
                "ID".cell().bold(true).justify(Justify::Right),
                "Title".cell().bold(true),
                "Status".cell().bold(true),
                "Created At".cell().bold(true),
                "Updated At".cell().bold(true),
                "Deleted At".cell().bold(true),
            ])
            .bold(true);

        let table_display = rows.display().unwrap();

        println!("{}", table_display);
    }

    pub fn seed_todos(&mut self) {
        println!("Enter the number of todos to be seeded: ");
        let mut num_todo = String::new();
        io::stdin()
            .read_line(&mut num_todo)
            .expect("Failed to parse number of todos to be seeded.");

        let num_todo: usize = match num_todo.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Failed to parse number of todos to be seeded.");
                return;
            }
        };

        for i in 1..num_todo {
            let todo = Todo {
                id: i,
                title: format!("Todo list {}", i),
                status: Status::Draft,
                created_at: Utc::now(),
                updated_at: Utc::now(),
                deleted_at: None,
            };

            self.add_todo(todo);
        }

        println!("Todo seeded successfully.");
    }

    pub fn export_to_csv(&self) {
        let mut csv_str = String::from("ID,Title,Status,CreatedAt,UpdatedAt,DeletedAt\n");

        for todo in &self.todos {
            let todo_str = format!(
                "{},{},{},{},{},{}\n",
                todo.id,
                todo.title,
                todo.status,
                todo.created_at.to_string(),
                todo.updated_at.to_string(),
                match todo.deleted_at {
                    Some(val) => val.to_string(),
                    None => "".to_string(),
                }
            );
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
