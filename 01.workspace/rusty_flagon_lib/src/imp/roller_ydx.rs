use crate::*;

impl Roller {
    pub(crate) fn ydx(&mut self, mut dice: u8, sides: u8) -> u16 {
        let mut result = 0u16;
        while dice > 0 {
            result += self.dx(sides) as u16;
            dice -= 1;
        }
        result
    }
}
