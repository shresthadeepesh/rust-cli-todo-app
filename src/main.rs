pub mod menu;
pub mod todo;

use menu::{display_menu, Choice};
use std::io;
use todo::{Status, Todo, TodoApp};

fn main() {
    let mut todo_app = TodoApp { todos: vec![] };

    let todo = Todo {
        id: &todo_app.todos.len() + 1,
        title: String::from("Todo list 1"),
        status: Status::Completed,
    };

    todo_app.todos.push(todo);

    display_menu();

    loop {
        let mut buff = String::new();
        io::stdin()
            .read_line(&mut buff)
            .expect("Failed to read line.");

        let buff: u8 = match buff.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match Choice::try_from(buff) {
            Ok(value) => match value {
                Choice::Exit => {
                    println!("Exiting from the program...");
                    break;
                }
                Choice::Insert => TodoApp::insert_todo(&mut todo_app),
                Choice::Update => TodoApp::update_todo(&mut todo_app),
                Choice::Delete => TodoApp::delete_todo(&mut todo_app),
                Choice::List => TodoApp::list_todos(&todo_app),
                Choice::DisplayMenu => display_menu(),
            },
            Err(e) => println!("Error: {}", e),
        }
    }
}
