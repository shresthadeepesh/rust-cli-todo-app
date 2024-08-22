pub fn display_menu() {
    println!("0. Exit.");
    println!("1. Press for insert a todo to the list.");
    println!("2. Press for update a todo from the list.");
    println!("3. Press for delete a todo from the list.");
    println!("4. Press for view todo list.");
    println!("5. Press for view the menu.");
}

#[derive(Debug)]
pub enum Choice {
    Exit = 0,
    Insert = 1,
    Update = 2,
    Delete = 3,
    List = 4,
    DisplayMenu = 5,
}

impl TryFrom<u8> for Choice {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Choice::Exit),
            1 => Ok(Choice::Insert),
            2 => Ok(Choice::Update),
            3 => Ok(Choice::Delete),
            4 => Ok(Choice::List),
            5 => Ok(Choice::DisplayMenu),
            _ => Err(format!("Invalid value: {}", value)),
        }
    }
}
