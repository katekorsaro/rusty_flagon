use crate::*;

/// A character.
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
    pub(crate) thac0_melee: u8,
    pub(crate) thac0_ranged: u8,
    pub(crate) save_death: u8,
    pub(crate) save_wands: u8,
    pub(crate) save_paralysis: u8,
    pub(crate) save_breath: u8,
    pub(crate) save_spell: u8,
    pub(crate) hp: u8,
    pub(crate) alignment: Alignment,
    pub(crate) starting_gold: u8,
    pub(crate) equipment: Vec<(String, u8)>,
    pub(crate) name: String,
    pub(crate) ac: i8,
}

impl Character {
    /// Returns the character's strength.
    pub fn strength(&self) -> u8 {
        self.strength
    }
    /// Returns the character's intelligence.
    pub fn intelligence(&self) -> u8 {
        self.intelligence
    }
    /// Returns the character's wisdom.
    pub fn wisdom(&self) -> u8 {
        self.wisdom
    }
    /// Returns the character's dexterity.
    pub fn dexterity(&self) -> u8 {
        self.dexterity
    }
    /// Returns the character's constitution.
    pub fn constitution(&self) -> u8 {
        self.constitution
    }
    /// Returns the character's charisma.
    pub fn charisma(&self) -> u8 {
        self.charisma
    }
    /// Returns the character's strength modifier.
    pub fn mod_strength(&self) -> i8 {
        self.mod_strength
    }
    /// Returns the character's intelligence modifier.
    pub fn mod_intelligence(&self) -> i8 {
        self.mod_intelligence
    }
    /// Returns the character's wisdom modifier.
    pub fn mod_wisdom(&self) -> i8 {
        self.mod_wisdom
    }
    /// Returns the character's dexterity modifier.
    pub fn mod_dexterity(&self) -> i8 {
        self.mod_dexterity
    }
    /// Returns the character's constitution modifier.
    pub fn mod_constitution(&self) -> i8 {
        self.mod_constitution
    }
    /// Returns the character's charisma modifier.
    pub fn mod_charisma(&self) -> i8 {
        self.mod_charisma
    }
    /// Returns the character's class.
    pub fn class(&self) -> Class {
        self.class
    }
    /// Returns the character's THAC0.
    pub fn thac0(&self) -> u8 {
        self.thac0
    }
    /// Returns the character's melee THAC0.
    pub fn thac0_melee(&self) -> u8 {
        self.thac0_melee
    }
    /// Returns the character's ranged THAC0.
    pub fn thac0_ranged(&self) -> u8 {
        self.thac0_ranged
    }
    /// Returns the character's saving throw against death.
    pub fn save_death(&self) -> u8 {
        self.save_death
    }
    /// Returns the character's saving throw against wands.
    pub fn save_wands(&self) -> u8 {
        self.save_wands
    }
    /// Returns the character's saving throw against paralysis.
    pub fn save_paralysis(&self) -> u8 {
        self.save_paralysis
    }
    /// Returns the character's saving throw against breath attacks.
    pub fn save_breath(&self) -> u8 {
        self.save_breath
    }
    /// Returns the character's saving throw against spells.
    pub fn save_spell(&self) -> u8 {
        self.save_spell
    }
    /// Returns the character's hit points.
    pub fn hp(&self) -> u8 {
        self.hp
    }
    /// Returns the character's alignment.
    pub fn alignment(&self) -> Alignment {
        self.alignment
    }
    /// Returns the character's starting gold.
    pub fn starting_gold(&self) -> u8 {
        self.starting_gold
    }
    /// Returns the character's equipment.
    pub fn equipment(&self) -> Vec<(String, u8)> {
        self.equipment.clone()
    }
    /// Returns the character's name.
    pub fn name(&self) -> String {
        self.name.clone()
    }
    /// Returns the character's armor class.
    pub fn ac(&self) -> i8 {
        self.ac
    }
}
