/// A character's alignment.
#[derive(Debug, Default, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Hash)]
pub enum E {
    /// No alignment.
    #[default]
    None,
    /// Lawful alignment.
    Law,
    /// Neutral alignment.
    Neutrality,
    /// Chaotic alignment.
    Chaos,
}
