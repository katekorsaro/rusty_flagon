use crate::*;

pub fn run() {
    let mut builder = Builder::new();
    let character = builder.build();
    match character {
        Ok(value) => print_character(&value),
        Err(_) => println!("An error occurred while generating a new character"),
    }
}
