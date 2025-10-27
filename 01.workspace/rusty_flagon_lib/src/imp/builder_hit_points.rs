use crate::*;

impl Builder {
    pub fn hit_points(&mut self) -> Result<(), FailedTo> {
        fn roll_hit_die(roller: &mut Roller, sides: u8) -> u8 {
            loop {
                let hp = roller.dx(sides);
                if hp > 2 {
                    break hp;
                }
            }
        }
        match self.character.class {
            Class::Cleric => self.character.hp = roll_hit_die(&mut self.roller, 6),
            Class::Dwarf => self.character.hp = roll_hit_die(&mut self.roller, 8),
            Class::Elf => self.character.hp = roll_hit_die(&mut self.roller, 6),
            Class::Fighter => self.character.hp = roll_hit_die(&mut self.roller, 8),
            Class::Halfling => self.character.hp = roll_hit_die(&mut self.roller, 6),
            Class::MagicUser => self.character.hp = roll_hit_die(&mut self.roller, 4),
            Class::Thief => self.character.hp = roll_hit_die(&mut self.roller, 4),
            Class::None => self.character.hp = 0,
        }
        self.character.hp = self
            .character
            .hp
            .checked_add_signed(self.character.mod_constitution)
            .unwrap_or(1);
        Ok(())
    }
}
