use crate::*;

/// A builder for creating [`Character`]s.
#[derive(Debug, Default, Clone)]
pub struct O {
    pub(crate) character: Character,
    pub(crate) roller: Roller,
}
