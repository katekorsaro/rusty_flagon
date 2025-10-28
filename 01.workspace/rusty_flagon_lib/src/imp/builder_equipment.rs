use crate::*;

impl Builder {
    pub(crate) fn equipment(&mut self) -> Result<(), FailedTo> {
        // cleric
        let cleric_pack_50gp: Vec<(&str, u8)> =
            vec![("Mace", 5), ("Chainmail", 40), ("Backpack", 5)];
        let cleric_pack_70gp: Vec<(&str, u8)> = vec![
            ("War hammer", 5),
            ("Plate mail", 60),
            ("Rations (standard, 7 days)", 5),
        ];
        let cleric_pack_90gp: Vec<(&str, u8)> = vec![
            ("Mace", 5),
            ("Plate mail", 60),
            ("Shield", 10),
            ("Backpack", 5),
            ("Rations (standard, 7 days)", 5),
            ("Torches (6)", 1),
            ("Waterskin", 1),
        ];
        let cleric_pack_110gp: Vec<(&str, u8)> = vec![
            ("War hammer", 5),
            ("Plate mail", 60),
            ("Shield", 10),
            ("Holy symbol", 25),
            ("Backpack", 5),
            ("Rations (standard, 7 days)", 5),
        ];
        // dwarf
        let dwarf_pack_50gp: Vec<(&str, u8)> =
            vec![("Battle axe", 7), ("Chainmail", 40), ("Backpack", 5)];
        let dwarf_pack_70gp: Vec<(&str, u8)> =
            vec![("War hammer", 5), ("Plate mail", 60), ("Backpack", 5)];
        let dwarf_pack_90gp: Vec<(&str, u8)> = vec![
            ("Sword", 10),
            ("Plate mail", 60),
            ("Shield", 10),
            ("Backpack", 5),
            ("Rations (standard, 7 days)", 5),
        ];
        let dwarf_pack_110gp: Vec<(&str, u8)> = vec![
            ("Battle axe", 7),
            ("Plate mail", 60),
            ("Shield", 10),
            ("Short bow", 25),
            ("Arrows (quiver of 20)", 5),
        ];
        // elf
        let elf_pack_50gp: Vec<(&str, u8)> = vec![
            ("Sword", 10),
            ("Leather Armor", 20),
            ("Shield", 10),
            ("Short bow", 25),
            ("Arrows (quiver of 20)", 5),
        ];
        let elf_pack_70gp: Vec<(&str, u8)> = vec![
            ("Sword", 10),
            ("Chainmail", 40),
            ("Short bow", 25),
            ("Arrows (quiver of 20)", 5),
        ];
        let elf_pack_90gp: Vec<(&str, u8)> = vec![
            ("Long bow", 40),
            ("Arrows (quiver of 20)", 5),
            ("Chainmail", 40),
            ("Dagger", 3),
        ];
        let elf_pack_110gp: Vec<(&str, u8)> = vec![
            ("Sword", 10),
            ("Plate mail", 60),
            ("Shield", 10),
            ("Long bow", 40),
            ("Arrows (quiver of 20)", 5),
        ];
        // fighter
        let fighter_pack_50gp: Vec<(&str, u8)> = vec![("Sword", 10), ("Chainmail", 40)];
        let fighter_pack_70gp: Vec<(&str, u8)> =
            vec![("Battle axe", 7), ("Plate mail", 60), ("Torches (6)", 1)];
        let fighter_pack_90gp: Vec<(&str, u8)> = vec![
            ("Sword", 10),
            ("Plate mail", 60),
            ("Shield", 10),
            ("Short bow", 25),
            ("Arrows (quiver of 20)", 5),
        ];
        let fighter_pack_110gp: Vec<(&str, u8)> = vec![
            ("Two-handed sword", 15),
            ("Plate mail", 60),
            ("Crossbow", 30),
            ("Crossbow bolts (case of 30)", 10),
        ];
        // halfling
        let halfling_pack_50gp: Vec<(&str, u8)> = vec![
            ("Short sword", 7),
            ("Leather Armor", 20),
            ("Shield", 10),
            ("Sling", 2),
            ("Backpack", 5),
            ("Rations (standard, 7 days)", 5),
        ];
        let halfling_pack_70gp: Vec<(&str, u8)> = vec![
            ("Short sword", 7),
            ("Chainmail", 40),
            ("Shield", 10),
            ("Sling", 2),
            ("Rations (standard, 7 days)", 5),
            ("Backpack", 5),
        ];
        let halfling_pack_90gp: Vec<(&str, u8)> = vec![
            ("Sword", 10),
            ("Chainmail", 40),
            ("Shield", 10),
            ("Short bow", 25),
            ("Arrows (quiver of 20)", 5),
        ];
        let halfling_pack_110gp: Vec<(&str, u8)> = vec![
            ("Sword", 10),
            ("Plate mail", 60),
            ("Shield", 10),
            ("Short bow", 25),
            ("Arrows (quiver of 20)", 5),
        ];
        // magic user
        let magic_user_pack_50gp: Vec<(&str, u8)> = vec![
            ("Dagger", 3),
            ("Backpack", 5),
            ("Lantern", 10),
            ("Oil (1 flask)", 2),
            ("Oil (1 flask)", 2),
            ("Rations (standard, 7 days)", 5),
            ("Tinder box", 3),
            ("Waterskin", 1),
            ("Holy water (vial)", 25),
        ];
        let magic_user_pack_70gp: Vec<(&str, u8)> = vec![
            ("Silver dagger", 30),
            ("Backpack", 5),
            ("Lantern", 10),
            ("Oil (1 flask)", 2),
            ("Oil (1 flask)", 2),
            ("Rations (iron, 7 days)", 15),
            ("Tinder box", 3),
            ("Waterskin", 1),
        ];
        let magic_user_pack_90gp: Vec<(&str, u8)> = vec![
            ("Silver dagger", 30),
            ("Holy water (vial)", 25),
            ("Grappling hook", 25),
            ("Rope (50')", 1),
            ("Backpack", 5),
            ("Torches (6)", 1),
            ("Tinder box", 3),
        ];
        let magic_user_pack_110gp: Vec<(&str, u8)> = vec![
            ("Silver dagger", 30),
            ("Holy water (vial)", 25),
            ("Holy water (vial)", 25),
            ("Grappling hook", 25),
            ("Rope (50')", 1),
            ("Backpack", 5),
        ];
        // thief
        let thief_pack_50gp: Vec<(&str, u8)> = vec![
            ("Leather Armor", 20),
            ("Thieves' tools", 25),
            ("Dagger", 3),
            ("Sack (small)", 1),
        ];
        let thief_pack_70gp: Vec<(&str, u8)> = vec![
            ("Leather Armor", 20),
            ("Thieves' tools", 25),
            ("Short sword", 7),
            ("Sling", 2),
            ("Backpack", 5),
            ("Crowbar", 10),
        ];
        let thief_pack_90gp: Vec<(&str, u8)> = vec![
            ("Leather Armor", 20),
            ("Thieves' tools", 25),
            ("Sword", 10),
            ("Short bow", 25),
            ("Arrows (quiver of 20)", 5),
            ("Backpack", 5),
        ];
        let thief_pack_110gp: Vec<(&str, u8)> = vec![
            ("Leather Armor", 20),
            ("Thieves' tools", 25),
            ("Short sword", 7),
            ("Short bow", 25),
            ("Arrows (quiver of 20)", 5),
            ("Grappling hook", 25),
            ("Rope (50')", 1),
            ("Backpack", 5),
        ];
        let wealth = self.character.starting_gold.saturating_sub(55);
        let wealth = wealth / 20 + 1;
        let starter_pack = match (self.character.class, wealth) {
            // cleric
            (Class::Cleric, 1) => cleric_pack_50gp,
            (Class::Cleric, 2) => cleric_pack_70gp,
            (Class::Cleric, 3) => cleric_pack_90gp,
            (Class::Cleric, 4) => cleric_pack_110gp,
            // dwarf
            (Class::Dwarf, 1) => dwarf_pack_50gp,
            (Class::Dwarf, 2) => dwarf_pack_70gp,
            (Class::Dwarf, 3) => dwarf_pack_90gp,
            (Class::Dwarf, 4) => dwarf_pack_110gp,
            // elf
            (Class::Elf, 1) => elf_pack_50gp,
            (Class::Elf, 2) => elf_pack_70gp,
            (Class::Elf, 3) => elf_pack_90gp,
            (Class::Elf, 4) => elf_pack_110gp,
            // fighter
            (Class::Fighter, 1) => fighter_pack_50gp,
            (Class::Fighter, 2) => fighter_pack_70gp,
            (Class::Fighter, 3) => fighter_pack_90gp,
            (Class::Fighter, 4) => fighter_pack_110gp,
            // halfling
            (Class::Halfling, 1) => halfling_pack_50gp,
            (Class::Halfling, 2) => halfling_pack_70gp,
            (Class::Halfling, 3) => halfling_pack_90gp,
            (Class::Halfling, 4) => halfling_pack_110gp,
            // magic user
            (Class::MagicUser, 1) => magic_user_pack_50gp,
            (Class::MagicUser, 2) => magic_user_pack_70gp,
            (Class::MagicUser, 3) => magic_user_pack_90gp,
            (Class::MagicUser, 4) => magic_user_pack_110gp,
            // thief
            (Class::Thief, 1) => thief_pack_50gp,
            (Class::Thief, 2) => thief_pack_70gp,
            (Class::Thief, 3) => thief_pack_90gp,
            (Class::Thief, 4) => thief_pack_110gp,
            _ => Vec::new(),
        };
        self.character.equipment = starter_pack
            .into_iter()
            .inspect(|item| {
                self.character.starting_gold = self.character.starting_gold.saturating_sub(item.1)
            })
            .map(|item| (item.0.to_string(), item.1))
            .collect();
        Ok(())
    }
}
