use serde::Serialize;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;

#[derive(Serialize)]
struct Tag {
    id: u8,
    name: String,
}

fn main() {
    start_menu();
    let mut option = String::new();
    println!("Selected: ");
    std::io::stdin()
        .read_line(&mut option)
        .expect("Failed to read line");
    let option: u32 = match option.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    launch_option(option);
}

fn start_menu() {
    println!("Welcome to notes app.");
    println!("Menu: Choose an option");
    println!("1. Add note");
    println!("2. List notes");
    println!("3. Add tag");
    println!("4. List tags");
    println!("5. Exit");
}

fn launch_option(option: u32) {
    println!("Option: {}", option);
    match option {
        1 => add_note(),
        2 => list_notes(),
        3 => match add_tag() {
            Ok(_) => (),
            Err(e) => println!("Error adding tag: {}", e),
        },
        4 => list_tags(),
        5 => exit(),
        _ => println!("Invalid option"),
    }
}

fn add_note() {
    println!("Add note");
}

fn list_notes() {
    println!("List notes");
}

fn add_tag() -> Result<(), Error> {
    println!("Add tag");
    let tag = Tag {
        id: 1,
        name: String::from("Work"),
    };
    println!("Tag id: {}", tag.id);
    println!("Tag name: {}", tag.name);
    let json_data = serde_json::to_string(&tag).unwrap();
    println!("JSON data: {}", json_data);
    let mut file = File::create("data/tags.json")?;
    file.write_all(json_data.as_bytes())?;
    println!("Successfully");
    Ok(())
}

fn list_tags() {
    println!("List tags");
}

fn exit() {
    println!("Exiting...");
}
