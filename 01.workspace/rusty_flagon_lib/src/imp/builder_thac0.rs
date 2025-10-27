use crate::*;

impl Builder {
    pub(crate) fn thac0(&mut self) -> Result<(), FailedTo> {
        self.character.thac0 = 19;
        Ok(())
    }
}
