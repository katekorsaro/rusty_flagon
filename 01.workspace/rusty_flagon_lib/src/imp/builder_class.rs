use crate::*;

impl Builder {
    fn can_be(&self, class: Class) -> bool {
        match class {
            Class::Cleric | Class::MagicUser | Class::Fighter | Class::Thief => true,
            Class::Elf => self.character.intelligence >= 9,
            Class::Dwarf => self.character.constitution >= 9,
            Class::Halfling => self.character.dexterity >= 9 && self.character.constitution >= 9,
            Class::None => true,
        }
    }
    fn should_be(&self, class: Class) -> bool {
        match class {
            Class::Fighter => true,
            Class::Cleric => self.character.wisdom >= 13,
            Class::MagicUser => self.character.intelligence >= 13,
            Class::Thief => self.character.dexterity >= 13,
            Class::Dwarf => self.character.strength >= 13,
            Class::Halfling => self.character.dexterity >= 13 && self.character.strength >= 13,
            Class::Elf => self.character.intelligence >= 13 && self.character.strength >= 13,
            _ => false,
        }
    }
    pub fn class(&mut self) -> Result<(), FailedTo> {
        let suggestion = [
            Class::Elf,
            Class::Halfling,
            Class::Dwarf,
            Class::Thief,
            Class::MagicUser,
            Class::Cleric,
            Class::Fighter,
        ]
        .into_iter()
        .map(|class| (class, self.can_be(class), self.should_be(class)))
        .find(|tuple| matches!(tuple, (_, true, true)))
        .map(|tuple| tuple.0);
        match suggestion {
            Some(class) => self.character.class = class,
            _ => self.character.class = Class::Fighter,
        }
        Ok(())
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    fn prepare_builder(str: u8, int: u8, wis: u8, dex: u8, con: u8, cha: u8) -> Builder {
        let mut builder = Builder::default();
        builder.character.strength = str;
        builder.character.intelligence = int;
        builder.character.wisdom = wis;
        builder.character.dexterity = dex;
        builder.character.constitution = con;
        builder.character.charisma = cha;
        builder
    }
    #[test]
    fn test_should_be_cleric() {
        let mut builder = prepare_builder(3, 3, 13, 3, 3, 3);
        let _ = builder.class();
        assert_eq!(builder.character.class, Class::Cleric);
    }
    #[test]
    fn test_should_be_magic_user() {
        let mut builder = prepare_builder(3, 13, 3, 3, 3, 3);
        let _ = builder.class();
        assert_eq!(builder.character.class, Class::MagicUser);
    }
    #[test]
    fn test_should_be_thief() {
        let mut builder = prepare_builder(3, 3, 3, 13, 3, 3);
        let _ = builder.class();
        assert_eq!(builder.character.class, Class::Thief);
    }
    #[test]
    fn test_should_be_dwarf() {
        let mut builder = prepare_builder(13, 3, 3, 3, 9, 3);
        let _ = builder.class();
        assert_eq!(builder.character.class, Class::Dwarf);
    }
    #[test]
    fn test_should_be_halfling() {
        let mut builder = prepare_builder(13, 3, 3, 13, 9, 3);
        let _ = builder.class();
        assert_eq!(builder.character.class, Class::Halfling);
    }
    #[test]
    fn test_should_be_elf() {
        let mut builder = prepare_builder(13, 13, 3, 3, 3, 3);
        let _ = builder.class();
        assert_eq!(builder.character.class, Class::Elf);
    }
    #[test]
    fn test_edge_case_all_stats_high() {
        let mut builder = prepare_builder(13, 13, 13, 13, 13, 13);
        let _ = builder.class();
        assert_eq!(builder.character.class, Class::Elf); // Elf is first in the list
    }
    #[test]
    fn test_edge_case_all_stats_low() {
        let mut builder = prepare_builder(8, 8, 8, 8, 8, 8);
        let _ = builder.class();
        assert_eq!(builder.character.class, Class::Fighter); // Should default to Fighter
    }
    #[test]
    fn test_edge_case_dwarf_can_be_false() {
        let mut builder = prepare_builder(13, 3, 3, 3, 8, 3); // con < 9
        let _ = builder.class();
        assert_ne!(builder.character.class, Class::Dwarf);
        assert_eq!(builder.character.class, Class::Fighter);
    }
    #[test]
    fn test_edge_case_elf_can_be_false() {
        let mut builder = prepare_builder(13, 8, 3, 3, 3, 3); // int < 9
        let _ = builder.class();
        assert_ne!(builder.character.class, Class::Elf);
    }
    #[test]
    fn test_edge_case_halfling_can_be_false() {
        let mut builder = prepare_builder(13, 3, 3, 13, 8, 3); // con < 9
        let _ = builder.class();
        assert_ne!(builder.character.class, Class::Halfling);
        let mut builder = prepare_builder(13, 3, 3, 8, 9, 3); // dex < 9
        let _ = builder.class();
        assert_ne!(builder.character.class, Class::Halfling);
    }
    #[test]
    fn test_edge_case_elf_and_halfling() {
        // High STR, INT, DEX, CON
        let mut builder = prepare_builder(13, 13, 3, 13, 9, 3);
        let _ = builder.class();
        assert_eq!(builder.character.class, Class::Elf); // Elf is checked before Halfling
    }
    #[test]
    fn test_edge_case_halfling_and_dwarf() {
        // High STR, DEX, CON
        let mut builder = prepare_builder(13, 3, 3, 13, 9, 3);
        let _ = builder.class();
        assert_eq!(builder.character.class, Class::Halfling); // Halfling is checked before Dwarf
    }
}
