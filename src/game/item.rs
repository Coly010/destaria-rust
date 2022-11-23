pub struct Weapon {
    pub name: String,
    pub damage: u32,
}

pub enum ArmourType {
    Head,
    Body,
    Legs,
    Hands,
    Feet
}

pub struct Armour {
    pub name: String,
    pub protection: u32,
    pub armour_type: ArmourType,
}

pub enum Item {
    Weapon(Weapon),
    Armour(Armour),
}

impl Item {
    pub fn name(&self) -> &String {
        match self {
            Item::Weapon(weapon) => &weapon.name,
            Item::Armour(armour) => &armour.name
        }
    }

    pub fn get_item_protection_or_damage(&self) -> u32 {
        match self {
            Item::Weapon(weapon) => weapon.damage,
            Item::Armour(armour) => armour.protection
        }
    }
}