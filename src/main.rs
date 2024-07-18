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
        3 => add_tag(),
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

fn add_tag() {
    println!("Add tag");
}

fn list_tags() {
    println!("List tags");
}

fn exit() {
    println!("Exiting...");
}
