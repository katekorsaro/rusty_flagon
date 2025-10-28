use crate::*;

impl Builder {
    pub(crate) fn alignment(&mut self) -> Result<(), FailedTo> {
        self.character.alignment = match self.roller.dx(3) {
            1 => Alignment::Law,
            2 => Alignment::Neutrality,
            3 => Alignment::Chaos,
            _ => Alignment::None,
        };
        Ok(())
    }
}

// #[cfg(test)]
// mod unit_tests { use super::*; }
