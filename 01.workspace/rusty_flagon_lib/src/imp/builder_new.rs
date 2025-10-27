use crate::*;

impl Builder {
    pub fn new() -> Self {
        Builder {
            character: Character::default(),
            roller: Roller::new(),
        }
    }
}
