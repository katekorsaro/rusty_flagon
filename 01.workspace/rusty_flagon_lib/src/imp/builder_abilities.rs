use crate::*;

impl Builder {
    pub(crate) fn abilities(&mut self) -> Result<(), FailedTo> {
        self.character.strength = self
            .roller
            .n3d6()
            .try_into()
            .map_err(|_| FailedTo::GenerateAbility)?;
        self.character.intelligence = self
            .roller
            .n3d6()
            .try_into()
            .map_err(|_| FailedTo::GenerateAbility)?;
        self.character.wisdom = self
            .roller
            .n3d6()
            .try_into()
            .map_err(|_| FailedTo::GenerateAbility)?;
        self.character.dexterity = self
            .roller
            .n3d6()
            .try_into()
            .map_err(|_| FailedTo::GenerateAbility)?;
        self.character.constitution = self
            .roller
            .n3d6()
            .try_into()
            .map_err(|_| FailedTo::GenerateAbility)?;
        self.character.charisma = self
            .roller
            .n3d6()
            .try_into()
            .map_err(|_| FailedTo::GenerateAbility)?;
        Ok(())
    }
}
