use crate::*;
use std::fmt::*;

impl Display for Character {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let equipment: Vec<_> = self.equipment.iter().map(|tuple| tuple.0.clone()).collect();
        let equipment = equipment.join(", ");
        let to_string = format!(
            r#"Class: {}
STR {} ({}) INT {} ({}) WIS {} ({}) DEX {} ({}) CON {} ({}) CHA {} ({})
Thac0: {} (melee {}, ranged {})
AC: {}
HP: {}
Saving Throws: D: {} W: {} P: {} B: {} S: {}
Alignment: {}
Equipment: {}
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
            self.ac,
            self.hp,
            self.save_death,
            self.save_wands,
            self.save_paralysis,
            self.save_breath,
            self.save_spell,
            match self.alignment {
                crate::Alignment::Law => "L",
                crate::Alignment::Neutrality => "N",
                crate::Alignment::Chaos => "C",
                _ => "-",
            },
            equipment,
            self.starting_gold,
        );
        write!(f, "{}", to_string)
    }
}
