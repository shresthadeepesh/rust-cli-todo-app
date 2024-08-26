pub fn display_menu() {
    println!("0. Exit.");
    println!("1. Press for view the menu.");
    println!("2. Press for insert a todo to the list.");
    println!("3. Press for update a todo from the list.");
    println!("4. Press for soft delete a todo from the list.");
    println!("5. Press for restore a todo from the list.");
    println!("6. Press for delete a todo from the list.");
    println!("7. Press for view todo list.");
    println!("8. Press for export to csv.");
    println!("9. Seed todos.");
    println!("10. Load todos from file.");
}

#[derive(Debug)]
pub enum Choice {
    Exit,
    DisplayMenu,
    Insert,
    Update,
    SoftDelete,
    Restore,
    Delete,
    List,
    ExportToCsv,
    SeedTodo,
    LoadFromFile,
}

impl TryFrom<u8> for Choice {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Choice::Exit),
            1 => Ok(Choice::DisplayMenu),
            2 => Ok(Choice::Insert),
            3 => Ok(Choice::Update),
            4 => Ok(Choice::SoftDelete),
            5 => Ok(Choice::Restore),
            6 => Ok(Choice::Delete),
            7 => Ok(Choice::List),
            8 => Ok(Choice::ExportToCsv),
            9 => Ok(Choice::SeedTodo),
            10 => Ok(Choice::LoadFromFile),
            _ => Err(format!("Invalid value: {}", value)),
        }
    }
}
