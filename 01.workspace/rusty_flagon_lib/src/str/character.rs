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
    pub(crate) save_death: u8,
    pub(crate) save_wands: u8,
    pub(crate) save_paralysis: u8,
    pub(crate) save_breath: u8,
    pub(crate) save_spell: u8,
    pub(crate) hp: u8,
    pub(crate) alignment: Alignment,
    pub(crate) starting_gold: u8,
}

impl Character {
    pub fn strength(&self) -> u8 {
        self.strength
    }
    pub fn intelligence(&self) -> u8 {
        self.intelligence
    }
    pub fn wisdom(&self) -> u8 {
        self.wisdom
    }
    pub fn dexterity(&self) -> u8 {
        self.dexterity
    }
    pub fn constitution(&self) -> u8 {
        self.constitution
    }
    pub fn charisma(&self) -> u8 {
        self.charisma
    }
    pub fn mod_strength(&self) -> i8 {
        self.mod_strength
    }
    pub fn mod_intelligence(&self) -> i8 {
        self.mod_intelligence
    }
    pub fn mod_wisdom(&self) -> i8 {
        self.mod_wisdom
    }
    pub fn mod_dexterity(&self) -> i8 {
        self.mod_dexterity
    }
    pub fn mod_constitution(&self) -> i8 {
        self.mod_constitution
    }
    pub fn mod_charisma(&self) -> i8 {
        self.mod_charisma
    }
    pub fn class(&self) -> Class {
        self.class
    }
    pub fn thac0(&self) -> u8 {
        self.thac0
    }
    pub fn save_death(&self) -> u8 {
        self.save_death
    }
    pub fn save_wands(&self) -> u8 {
        self.save_wands
    }
    pub fn save_paralysis(&self) -> u8 {
        self.save_paralysis
    }
    pub fn save_breath(&self) -> u8 {
        self.save_breath
    }
    pub fn save_spell(&self) -> u8 {
        self.save_spell
    }
    pub fn hp(&self) -> u8 {
        self.hp
    }
    pub fn alignment(&self) -> Alignment {
        self.alignment
    }
    pub fn starting_gold(&self) -> u8 {
        self.starting_gold
    }
}
