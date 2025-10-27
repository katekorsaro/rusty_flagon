use crate::*;

impl Builder {
    pub fn thac0(&mut self) -> Result<(), FailedTo> {
        self.character.thac0 = 19;
        Ok(())
    }
}
