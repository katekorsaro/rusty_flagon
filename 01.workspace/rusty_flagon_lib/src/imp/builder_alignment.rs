use crate::*;

impl Builder {
    pub fn alignment(&mut self) -> Result<(), FailedTo> {
        self.character.alignment = match self.roller.dx(3) {
            1 => Alignment::Law,
            2 => Alignment::Neutral,
            3 => Alignment::Chaos,
            _ => Alignment::None,
        };
        Ok(())
    }
}

// #[cfg(test)]
// mod unit_tests { use super::*; }
