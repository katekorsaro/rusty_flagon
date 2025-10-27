use crate::*;

impl Builder {
    pub fn starting_gold(&mut self) -> Result<(), FailedTo> {
        self.character.starting_gold = (self.roller.ydx(3, 6) * 10)
            .try_into()
            .map_err(|_| FailedTo::GenerateStartingGold)?;
        Ok(())
    }
}
