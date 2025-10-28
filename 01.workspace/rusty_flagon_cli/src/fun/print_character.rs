use crate::*;
use colored::*;

fn format_class(class: Class) -> ColoredString {
    let str = match class {
        Class::Cleric => "Cleric",
        Class::Dwarf => "Dwarf",
        Class::Elf => "Elf",
        Class::Halfling => "Hafling",
        Class::Fighter => "Fighter",
        Class::MagicUser => "Magic User",
        Class::Thief => "Thief",
        Class::None => "None",
    };
    str.bright_blue().bold()
}

fn format_ability_value(value: u8) -> ColoredString {
    if value < 9 {
        value.to_string().red().bold()
    } else if value < 13 {
        value.to_string().white().bold()
    } else {
        value.to_string().green().bold()
    }
}

fn format_modifier_value(value: i8) -> ColoredString {
    if value < 0 {
        value.to_string().bright_red().bold()
    } else if value == 0 {
        value.to_string().bright_white().bold()
    } else {
        value.to_string().bright_green().bold()
    }
}

fn format_saving_throws(value: u8) -> ColoredString {
    value.to_string().bright_white().bold()
}

fn format_hp(value: u8) -> ColoredString {
    value.to_string().bright_magenta().bold()
}

fn format_thac0(value: u8) -> ColoredString {
    value.to_string().bright_white().bold()
}

fn format_alignment(value: Alignment) -> ColoredString {
    match value {
        Alignment::Law => "Law".bright_green().bold(),
        Alignment::Neutrality => "Neutrality".bright_white().bold(),
        Alignment::Chaos => "Chaos".bright_red().bold(),
        _ => "-".bright_white().bold(),
    }
}

fn format_starting_gold(value: u8) -> ColoredString {
    value.to_string().bright_yellow().bold()
}

pub fn run(character: &Character) {
    println!("Class {}", format_class(character.class()));
    println!("Alignment: {}", format_alignment(character.alignment()));
    print!("STR {} ", format_ability_value(character.strength()));
    print!("({})  ", format_modifier_value(character.mod_strength()));
    print!("INT {} ", format_ability_value(character.intelligence()));
    print!(
        "({})  ",
        format_modifier_value(character.mod_intelligence())
    );
    print!("WIS {} ", format_ability_value(character.wisdom()));
    print!("({})  ", format_modifier_value(character.mod_wisdom()));
    print!("DEX {} ", format_ability_value(character.dexterity()));
    print!("({})  ", format_modifier_value(character.mod_dexterity()));
    print!("CON {} ", format_ability_value(character.constitution()));
    print!(
        "({})  ",
        format_modifier_value(character.mod_constitution())
    );
    print!("CHA {} ", format_ability_value(character.charisma()));
    println!("({})", format_modifier_value(character.mod_charisma()));
    print!("Death {}  ", format_saving_throws(character.save_death()));
    print!("Wands {}  ", format_saving_throws(character.save_wands()));
    print!(
        "Paralysis {}  ",
        format_saving_throws(character.save_paralysis())
    );
    print!("Breath {}  ", format_saving_throws(character.save_breath()));
    println!("Spell {}", format_saving_throws(character.save_spell()));
    println!("HP {}", format_hp(character.hp()));
    println!(
        "Thac0 {} (melee: {}  ranged: {})",
        format_thac0(character.thac0()),
        format_thac0(character.thac0_melee()),
        format_thac0(character.thac0_ranged())
    );
    println!(
        "Starting Gold: {}",
        format_starting_gold(character.starting_gold())
    );
}
