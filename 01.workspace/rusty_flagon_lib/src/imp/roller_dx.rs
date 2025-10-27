use crate::*;

impl Roller {
    pub(crate) fn dx(&mut self, sides: u8) -> u8 {
        self.rng.random_range(1..=sides)
    }
}
