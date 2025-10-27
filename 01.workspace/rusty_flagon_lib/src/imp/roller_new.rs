use crate::*;

impl Roller {
    pub(crate) fn new() -> Self {
        Self { rng: rand::rng() }
    }
}

// #[cfg(test)]
// mod unit_tests { use super::*; }
