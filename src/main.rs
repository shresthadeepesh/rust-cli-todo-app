use std::{sync::mpsc, thread, time::Duration};

pub mod todo_app;

fn main() {
    todo_app::todo_app();

    // let (tx, rx) = mpsc::channel();

    // let tx1 = tx.clone();
    // thread::spawn(move || {
    //     let txt = vec![
    //         String::from("Hello World! 1"),
    //         String::from("Hello World! 2"),
    //         String::from("Hello World! 3"),
    //         String::from("Hello World! 4"),
    //         String::from("Hello World! 5"),
    //     ];

    //     for text in txt {
    //         tx1.send(text).unwrap();
    //         thread::sleep(Duration::from_secs(2));
    //     }
    // });

    // thread::spawn(move || {
    //     let txt = vec![
    //         String::from("Hello World! 11"),
    //         String::from("Hello World! 22"),
    //         String::from("Hello World! 33"),
    //         String::from("Hello World! 44"),
    //         String::from("Hello World! 55"),
    //     ];

    //     for text in txt {
    //         tx.send(text).unwrap();
    //         thread::sleep(Duration::from_secs(2));
    //     }
    // });

    // for received in rx {
    //     println!("Got: {received}");
    // }
}
