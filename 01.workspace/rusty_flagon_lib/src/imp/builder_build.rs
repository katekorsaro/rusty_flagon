use crate::*;

impl Builder {
    pub fn build(&mut self) -> Result<Character, FailedTo> {
        self.abilities()?;
        self.class()?;
        self.modifiers()?;
        self.thac0()?;
        self.saving_throws()?;
        self.hit_points()?;
        self.alignment()?;
        self.starting_gold()?;
        self.equipment()?;
        Ok(self.character.clone())
    }
}
