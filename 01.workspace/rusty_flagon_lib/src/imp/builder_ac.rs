use crate::*;

impl Builder {
    pub(crate) fn ac(&mut self) -> Result<(), FailedTo> {
        self.character.ac = 9;
        let armour = self
            .character
            .equipment
            .iter()
            .map(|item| item.0.clone())
            .find(|item| item == "Leather Armor" || item == "Chainmail" || item == "Plate mail");
        let shield = self
            .character
            .equipment
            .iter()
            .map(|item| item.0.clone())
            .find(|item| item == "Shield");
        let mut ac = 9;
        if let Some(value) = armour {
            ac = match value.as_ref() {
                "Leather Armor" => 7,
                "Chainmail" => 5,
                "Plate mail" => 3,
                _ => 9,
            };
        }
        if shield.is_some() {
            ac -= 1;
        }
        ac -= self.character.mod_dexterity;
        self.character.ac = ac;
        Ok(())
    }
}
