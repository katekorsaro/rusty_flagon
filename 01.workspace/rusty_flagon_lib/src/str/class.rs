/// The character's class.
#[derive(Debug, Default, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Hash)]
pub enum E {
    /// No class.
    #[default]
    None,
    /// The cleric class.
    Cleric,
    /// The dwarf class.
    Dwarf,
    /// The elf class.
    Elf,
    /// The fighter class.
    Fighter,
    /// The halfling class.
    Halfling,
    /// The magic-user class.
    MagicUser,
    /// The thief class.
    Thief,
}
