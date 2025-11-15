use crate::*;

pub fn run() {
    let mut builder = Builder::new();
    let character = builder.build();
    match character {
        Ok(character) => {
            let name = character.name().replace(" ", "_").to_lowercase();
            let class = match character.class() {
                Class::Cleric => "cleric".to_string(),
                Class::Dwarf => "dwarf".to_string(),
                Class::Elf => "elf".to_string(),
                Class::Fighter => "fighter".to_string(),
                Class::Halfling => "halffling".to_string(),
                Class::MagicUser => "magic_user".to_string(),
                Class::Thief => "thief".to_string(),
                _ => panic!(),
            };
            let filename = format!("./{}_({}).md", name, class);
            std::fs::write(&filename, character.to_string()).expect("file is written");
        }
        Err(e) => println!("{e:?}"),
    }
}
