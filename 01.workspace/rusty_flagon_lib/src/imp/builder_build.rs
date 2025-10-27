use crate::*;

impl Builder {
    pub fn build(&mut self) -> Result<Character, FailedTo> {
        // 1. character abilities
        self.abilities();
        // Returning final character instance
        Ok(self.character.clone())
    }
}
