#[derive(Debug, Default, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Hash)]
pub enum E {
    #[default]
    None,
    Cleric,
    Dwarf,
    Elf,
    Fighter,
    Halfling,
    MagicUser,
    Thief,
}
