use serde::{Deserialize, Serialize};
use std::io::prelude::*;
use std::io::Write;

#[derive(Serialize, Deserialize)]

struct Tag {
    id: u32,
    name: String,
}

const FILE_TAG: &str = "data/tags.json";

/////////////
fn read_file_content(file_name: String) -> Vec<Tag> {
    println!("Leyendo {}...", file_name);
    // Leer el contenido actual del archivo
    let mut file = std::fs::OpenOptions::new().read(true).open(FILE_TAG).expect("No se pudo abrir el archivo");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("No se pudo leer el archivo");

    // Deserializar el contenido en un array
    let tags: Vec<Tag> = match serde_json::from_str(&contents) {
        Ok(tags) => tags,
        Err(_) => vec![],
    };

    // Imprimir tags en formato JSON
    println!("Tags:");
    let json_tags = serde_json::to_string_pretty(&tags).expect("No se pudo serializar a JSON");
    println!("{}", json_tags);

    // Retornar tags
    // json_tags
    tags
}

////////////


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

fn add_tag() -> Result<(), Box<dyn std::error::Error>> {
    println!("Add tag");
    let mut name: String = String::new();
    std::io::stdin().read_line(&mut name)?;
    name = name.trim().to_string();

    // Leer el contenido actual del archivo
    let mut file = std::fs::OpenOptions::new().read(true).open(FILE_TAG)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Deserializar el contenido en un array
    let mut tags: Vec<Tag> = match serde_json::from_str(&contents) {
        Ok(tags) => tags,
        Err(_) => vec![],
    };

    // Contar el número de elementos actuales
    let new_id = (tags.len() as u32) + 1;

    // Crear el nuevo tag con el id incrementado
    let tag = Tag {
        id: new_id,
        name: String::from(name),
    };

    println!("Tag id: {}", tag.id);
    println!("Tag name: {}", tag.name);
    let json_data = serde_json::to_string(&tag).unwrap();
    println!("JSON data: {}", json_data);

    // Agregar el nuevo tag al array
    tags.push(tag);

    // Serializar el array de nuevo a JSON
    let new_json_data = serde_json::to_string_pretty(&tags).unwrap();

    // Escribir el nuevo JSON en el archivo
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(FILE_TAG)?;
    file.write_all(new_json_data.as_bytes())?;
    println!("Successfully written to file");
    Ok(())
}

fn list_tags(){
    println!("Lista de etiquetas");
    let contenido = read_file_content(FILE_TAG.to_string());
}

fn exit() {
    println!("Exiting...");
}
