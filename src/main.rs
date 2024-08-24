pub mod menu;
pub mod todo;

use menu::{display_menu, Choice};
use std::io;
use todo::TodoApp;

fn main() {
    let mut todo_app = TodoApp::new();

    todo_app.seed_todos();
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
                Choice::Insert => todo_app.insert_todo(),
                Choice::Update => todo_app.update_todo(),
                Choice::Delete => todo_app.delete_todo(),
                Choice::List => todo_app.list_todos(),
                Choice::DisplayMenu => display_menu(),
                Choice::ExportToCsv => todo_app.export_to_csv(),
            },
            Err(e) => println!("Error: {}", e),
        }
    }
}
