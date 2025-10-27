use crate::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct O {
    pub(crate) strength: u8,
    pub(crate) intelligence: u8,
    pub(crate) wisdom: u8,
    pub(crate) dexterity: u8,
    pub(crate) constitution: u8,
    pub(crate) charisma: u8,
    pub(crate) mod_strength: i8,
    pub(crate) mod_intelligence: i8,
    pub(crate) mod_wisdom: i8,
    pub(crate) mod_dexterity: i8,
    pub(crate) mod_constitution: i8,
    pub(crate) mod_charisma: i8,
    pub(crate) class: Class,
    pub(crate) thac0: u8,
}
