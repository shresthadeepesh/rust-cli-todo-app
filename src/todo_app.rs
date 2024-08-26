pub mod menu;
pub mod todo;

use menu::{display_menu, Choice};
use std::io;
use todo::{ITodoApp, TodoApp};

pub fn todo_app() {
    let mut todo_app = TodoApp::new();

    display_menu();

    loop {
        let mut buff = String::new();
        io::stdin()
            .read_line(&mut buff)
            .expect("Failed to read line.");

        if buff.trim().to_lowercase() == "q" {
            println!("Exiting from the program...");
            break;
        }

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
                Choice::Insert => todo_app.insert_todo(),
                Choice::Update => todo_app.update_todo(),
                Choice::SoftDelete => todo_app.soft_delete_todo(),
                Choice::Restore => todo_app.restore_todo(),
                Choice::Delete => todo_app.delete_todo(),
                Choice::List => todo_app.list_todos(),
                Choice::DisplayMenu => display_menu(),
                Choice::ExportToCsv => todo_app.export_to_csv(),
                Choice::SeedTodo => todo_app.seed_todos(),
                Choice::LoadFromFile => todo_app.load_from_file(),
                Choice::CountTodo => todo_app.todos_len(),
            },
            Err(e) => println!("Error: {}", e),
        }
    }
}
