/// An error that can occur when building a character.
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Hash)]
pub enum E {
    /// Failed to generate an ability score.
    GenerateAbility,
    /// Failed to generate starting gold.
    GenerateStartingGold,
}
