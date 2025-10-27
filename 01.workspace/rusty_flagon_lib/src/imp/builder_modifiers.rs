use crate::*;

impl Builder {
    pub fn modifiers(&mut self) -> Result<(), FailedTo> {
        fn modifier(ability: u8) -> i8 {
            match ability {
                3 => -3,
                4..=5 => -2,
                6..=8 => -1,
                9..=12 => 0,
                13..=15 => 1,
                16..=17 => 2,
                18 => 3,
                _ => 0,
            }
        }
        self.character.mod_strength = modifier(self.character.strength);
        self.character.mod_intelligence = modifier(self.character.intelligence);
        self.character.mod_wisdom = modifier(self.character.wisdom);
        self.character.mod_dexterity = modifier(self.character.dexterity);
        self.character.mod_constitution = modifier(self.character.constitution);
        self.character.mod_charisma = modifier(self.character.charisma);
        Ok(())
    }
}
