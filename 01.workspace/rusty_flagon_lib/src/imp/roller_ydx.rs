use crate::*;

impl Roller {
    pub(crate) fn ydx(&mut self, dice: u8, sides: u8) -> u16 {
        let mut result = 0u16;
        for _ in 1..=dice {
            result += self.dx(sides) as u16;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ydx_3d6() {
        let mut roller = Roller::new();
        for _ in 0..100 {
            let result = roller.ydx(3, 6);
            assert!((3..=18).contains(&result));
        }
    }
    #[test]
    fn test_ydx_1d20() {
        let mut roller = Roller::new();
        for _ in 0..100 {
            let result = roller.ydx(1, 20);
            assert!((1..=20).contains(&result));
        }
    }
    #[test]
    fn test_ydx_10d10() {
        let mut roller = Roller::new();
        for _ in 0..1000 {
            let result = roller.ydx(10, 10);
            assert!((10..=100).contains(&result));
        }
    }
}
