use chrono::{DateTime, NaiveDateTime, Utc};
use cli_table::{format::Justify, Cell, Style, Table};
use core::fmt;
use std::{
    fs::{read_to_string, File},
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

pub trait ITodoApp {
    fn new() -> Self;
    fn add_todo(&mut self, todo: Todo);
    fn todos_len(&self);
    fn generate_id(&self) -> usize;
    fn insert_todo(&mut self);
    fn update_todo(&mut self);
    fn soft_delete_todo(&mut self);
    fn restore_todo(&mut self);
    fn delete_todo(&mut self);
    fn list_todos(&self);
    fn seed_todos(&mut self);
    fn export_to_csv(&self);
    fn load_from_file(&mut self);
}

pub struct TodoApp {
    todos: Vec<Todo>,
}

impl ITodoApp for TodoApp {
    fn new() -> Self {
        TodoApp { todos: Vec::new() }
    }

    fn add_todo(&mut self, todo: Todo) {
        self.todos.push(todo);
    }

    fn todos_len(&self) {
        println!("{}", self.todos.len());
    }

    fn generate_id(&self) -> usize {
        if let Some(max_id) = self.todos.iter().map(|todo| todo.id).max() {
            max_id + 1
        } else {
            1
        }
    }

    fn insert_todo(&mut self) {
        println!("Enter the todo list: ");

        let mut todo_input = String::new();
        io::stdin()
            .read_line(&mut todo_input)
            .expect("Failed to read line.");

        let todo = Todo {
            id: self.generate_id(),
            title: todo_input.trim().to_string(),
            status: Status::Draft,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        };

        self.todos.push(todo);

        println!("The todo has been added to the list.");
    }

    fn update_todo(&mut self) {
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

    fn soft_delete_todo(&mut self) {
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

    fn restore_todo(&mut self) {
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

    fn delete_todo(&mut self) {
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

    fn list_todos(&self) {
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

    fn seed_todos(&mut self) {
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

    fn export_to_csv(&self) {
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

    fn load_from_file(&mut self) {
        let todos: Vec<Todo> = read_to_string("todos.csv")
            .unwrap()
            .lines()
            .skip(1)
            .map(|line| {
                let fields: Vec<&str> = line.split(",").collect();
                Todo {
                    id: fields[0].parse().unwrap_or_default(),
                    title: fields[1].to_string(),
                    status: if fields[2].to_string() == "Draft" {
                        Status::Draft
                    } else {
                        Status::Completed
                    },
                    created_at: fields[3].parse::<DateTime<Utc>>().unwrap(),
                    updated_at: fields[4].parse::<DateTime<Utc>>().unwrap(),
                    deleted_at: if !fields[5].is_empty() {
                        Some(fields[4].parse::<DateTime<Utc>>().unwrap())
                    } else {
                        None
                    },
                }
            })
            .collect();

        if todos.len() > 0 {
            self.todos = todos;
            println!("Todos loaded from the file.");
        }
    }
}
