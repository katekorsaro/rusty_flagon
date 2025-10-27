use crate::*;

impl Roller {
    pub(crate) fn dx(&mut self, sides: u8) -> u8 {
        self.rng.random_range(1..=sides)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dx_d6() {
        let mut roller = Roller::new();
        for _ in 0..100 {
            let result = roller.dx(6);
            assert!((1..=6).contains(&result));
        }
    }
    #[test]
    fn test_dx_d20() {
        let mut roller = Roller::new();
        for _ in 0..100 {
            let result = roller.dx(20);
            assert!((1..=20).contains(&result));
        }
    }
    #[test]
    fn test_dx_d100() {
        let mut roller = Roller::new();
        for _ in 0..1000 {
            let result = roller.dx(100);
            assert!((1..=100).contains(&result));
        }
    }
}
