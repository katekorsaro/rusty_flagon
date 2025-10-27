use crate::*;

impl Builder {
    pub fn saving_throws(&mut self) -> Result<(), FailedTo> {
        fn from_slice(values: &[u8], character: &mut Character) {
            character.save_death = values[0];
            character.save_wands = values[1];
            character.save_paralysis = values[2];
            character.save_breath = values[3];
            character.save_spell = values[4];
        }
        match self.character.class {
            Class::Cleric => from_slice(&[11, 12, 14, 16, 15], &mut self.character),
            Class::Dwarf => from_slice(&[8, 9, 10, 13, 12], &mut self.character),
            Class::Elf => from_slice(&[12, 13, 13, 15, 15], &mut self.character),
            Class::Fighter => from_slice(&[12, 13, 14, 15, 16], &mut self.character),
            Class::Halfling => from_slice(&[8, 9, 10, 13, 12], &mut self.character),
            Class::MagicUser => from_slice(&[13, 14, 13, 16, 15], &mut self.character),
            Class::Thief => from_slice(&[13, 14, 13, 16, 15], &mut self.character),
            Class::None => (),
        }
        Ok(())
    }
}

// #[cfg(test)]
// mod unit_tests { use super::*; }
