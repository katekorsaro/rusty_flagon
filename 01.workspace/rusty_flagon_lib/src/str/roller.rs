use crate::*;

/// A dice roller.
#[derive(Debug, Default, Clone)]
pub struct O {
    pub(crate) rng: rngs::ThreadRng,
}
