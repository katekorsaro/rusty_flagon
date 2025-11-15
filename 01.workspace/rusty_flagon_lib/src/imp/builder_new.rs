use crate::*;

impl Builder {
    /// Creates a new `Builder`.
    pub fn new() -> Self {
        Builder {
            character: Character::default(),
            roller: Roller::new(),
        }
    }
}
