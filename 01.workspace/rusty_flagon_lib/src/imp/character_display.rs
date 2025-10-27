use crate::*;
use std::fmt::*;

impl Display for Character {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let to_string = format!(
            r#"Class: {}
STR {} ({}) INT {} ({}) WIS {} ({}) DEX {} ({}) CON {} ({}) CHA {} ({})
Thac0: {} (melee {}, ranged {})
HP: {}
Saving Throwns: D: {} W: {} P: {} B: {} S: {}
Alignment: {}
Starting Gold: {}
"#,
            match self.class {
                Class::Cleric => "Cleric",
                Class::Dwarf => "Dwarf",
                Class::Elf => "Elf",
                Class::Halfling => "Halfling",
                Class::Fighter => "Fighter",
                Class::MagicUser => "Magic User",
                Class::Thief => "Thief",
                Class::None => "-",
            },
            self.strength,
            self.mod_strength,
            self.intelligence,
            self.mod_intelligence,
            self.wisdom,
            self.mod_wisdom,
            self.dexterity,
            self.mod_dexterity,
            self.constitution,
            self.mod_constitution,
            self.charisma,
            self.mod_charisma,
            self.thac0,
            self.thac0_melee,
            self.thac0_ranged,
            self.hp,
            self.save_death,
            self.save_wands,
            self.save_paralysis,
            self.save_breath,
            self.save_spell,
            match self.alignment {
                crate::Alignment::Law => "L",
                crate::Alignment::Neutral => "N",
                crate::Alignment::Chaos => "C",
                _ => "-",
            },
            self.starting_gold,
        );
        write!(f, "{}", to_string)
    }
}
