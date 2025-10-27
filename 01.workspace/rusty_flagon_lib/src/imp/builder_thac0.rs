use crate::*;

impl Builder {
    pub(crate) fn thac0(&mut self) -> Result<(), FailedTo> {
        self.character.thac0 = 19;
        self.character.thac0_melee = match self.character.mod_strength {
            -3 => 22,
            -2 => 21,
            -1 => 20,
            0 => 19,
            1 => 18,
            2 => 17,
            3 => 16,
            _ => self.character.thac0,
        };
        self.character.thac0_ranged = match self.character.mod_dexterity {
            -3 => 22,
            -2 => 21,
            -1 => 20,
            0 => 19,
            1 => 18,
            2 => 17,
            3 => 16,
            _ => self.character.thac0,
        };
        Ok(())
    }
}
