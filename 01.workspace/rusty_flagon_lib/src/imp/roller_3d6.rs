use crate::*;

impl Roller {
    pub(crate) fn n3d6(&mut self) -> u16 {
        self.ydx(3, 6)
    }
}
