use crate::*;

impl Roller {
    pub(crate) fn d6(&mut self) -> u8 {
        self.dx(6)
    }
}

// #[cfg(test)]
// mod unit_tests { use super::*; }
