use super::item::{ArmourType, Item};

pub struct BattleGear<'a> {
    pub head: Option<&'a Item>,
    pub body: Option<&'a Item>,
    pub legs: Option<&'a Item>,
    pub hands: Option<&'a Item>,
    pub feet: Option<&'a Item>,
    pub weapon: Option<&'a Item>,
}

impl<'a> BattleGear<'a> {
    pub fn new() -> BattleGear<'a> {
        BattleGear {
            head: None,
            body: None,
            legs: None,
            hands: None,
            feet: None,
            weapon: None,
        }
    }

    pub fn calculate_protection(&self) -> u32 {
        let mut protection: u32 = 0;

        let battle_gear = &self;
        if let Some(item) = battle_gear.head {
            protection += item.get_item_protection_or_damage();
        }
        if let Some(item) = battle_gear.body {
            protection += item.get_item_protection_or_damage();
        }
        if let Some(item) = battle_gear.legs {
            protection += item.get_item_protection_or_damage();
        }
        if let Some(item) = battle_gear.hands {
            protection += item.get_item_protection_or_damage();
        }
        if let Some(item) = battle_gear.feet {
            protection += item.get_item_protection_or_damage();
        }

        protection
    }

    pub fn calculate_damage(&self) -> u32 {
        let mut damage: u32 = 0;

        let battle_gear = &self;
        if let Some(item) = battle_gear.weapon {
            damage += item.get_item_protection_or_damage();
        }

        damage
    }

    pub fn equip_item<'b>(&'b mut self, item: &'a Item) -> Option<&'a Item> {
        let mut item_to_return_to_inventory: Option<&'a Item> = None;
        match item {
            Item::Armour(armour) => match armour.armour_type {
                ArmourType::Head => {
                    if let Some(item) = self.head {
                        item_to_return_to_inventory = Some(item);
                    }
                    self.head = Some(item)
                }
                ArmourType::Body => {
                    if let Some(item) = self.body {
                        item_to_return_to_inventory = Some(item);
                    }
                    self.body = Some(item)
                }
                ArmourType::Legs => {
                    if let Some(item) = self.legs {
                        item_to_return_to_inventory = Some(item);
                    }
                    self.legs = Some(item)
                }
                ArmourType::Hands => {
                    if let Some(item) = self.hands {
                        item_to_return_to_inventory = Some(item);
                    }
                    self.hands = Some(item)
                }
                ArmourType::Feet => {
                    if let Some(item) = self.feet {
                        item_to_return_to_inventory = Some(item);
                    }
                    self.feet = Some(item)
                }
            },
            Item::Weapon(_) => {
                if let Some(item) = self.weapon {
                    item_to_return_to_inventory = Some(item);
                }
                self.weapon = Some(item)
            }
        }

        item_to_return_to_inventory
    }

    pub fn unequip_item<'b>(&'b mut self, item: &'a Item) {
        match item {
            Item::Armour(armour) => match armour.armour_type {
                ArmourType::Head => self.head = None,
                ArmourType::Body => self.body = None,
                ArmourType::Legs => self.legs = None,
                ArmourType::Hands => self.hands = None,
                ArmourType::Feet => self.feet = None,
            },
            Item::Weapon(_) => self.weapon = None,
        }
    }

    pub fn has_items_equipped(&self) -> bool {
        let mut has_equipped_items = false;
        if let Some(_) = self.head {
            has_equipped_items = true;
        }
        if let Some(_) = self.body {
            has_equipped_items = true;
        }
        if let Some(_) = self.legs {
            has_equipped_items = true;
        }
        if let Some(_) = self.hands {
            has_equipped_items = true;
        }
        if let Some(_) = self.feet {
            has_equipped_items = true;
        }
        if let Some(_) = self.weapon {
            has_equipped_items = true;
        }

        has_equipped_items
    }

    pub fn get_all_items_equipped_as_vec<'b>(&'b self) -> Vec<&'a Item> {
        let mut items: Vec<&Item> = vec![];

        if let Some(item) = self.head {
            items.push(item);
        }
        if let Some(item) = self.body {
            items.push(item);
        }
        if let Some(item) = self.legs {
            items.push(item);
        }
        if let Some(item) = self.hands {
            items.push(item);
        }
        if let Some(item) = self.feet {
            items.push(item);
        }
        if let Some(item) = self.weapon {
            items.push(item);
        }

        items
    }
}

pub struct NPC<'a> {
    pub name: String,
    pub level: u32,
    pub strength: u32,
    battle_gear: BattleGear<'a>,
}

impl<'a> NPC<'a> {
    pub fn new(name: String) -> NPC<'a> {
        NPC {
            name,
            level: 1,
            strength: 1,
            battle_gear: BattleGear::new(),
        }
    }

    pub fn get_battle_gear(&self) -> &BattleGear {
        &self.battle_gear
    }

    pub fn equip_item<'b>(&'b mut self, item: &'a Item) {
        self.battle_gear.equip_item(item);
    }

    pub fn unequip_item<'b>(&'b mut self, item: &'a Item) {
        self.battle_gear.unequip_item(item);
    }
}

pub struct Player<'a> {
    pub name: String,
    pub level: u32,
    pub exp: u32,
    pub money: u32,
    pub battles_won: u32,
    pub battles_lost: u32,
    pub strength: u32,
    pub inventory: Vec<&'a Item>,
    pub battle_gear: BattleGear<'a>,
}

impl<'a> Player<'a> {
    pub fn new() -> Player<'a> {
        Player {
            name: String::from(""),
            level: 1,
            exp: 0,
            money: 0,
            battles_won: 0,
            battles_lost: 0,
            strength: 1,
            inventory: Vec::new(),
            battle_gear: BattleGear::new(),
        }
    }

    pub fn add_item_to_inventory<'b>(&'b mut self, item: &'a Item) {
        self.inventory.push(item);
    }

    pub fn remove_item_from_inventory<'b>(&'b mut self, item: &'a Item) {
        let item_name_to_remove = match item {
            Item::Weapon(weapon) => &weapon.name,
            Item::Armour(armour) => &armour.name,
        };

        let index = self.inventory.iter().position(|r| match r {
            Item::Weapon(weapon) => weapon.name.eq(item_name_to_remove),
            Item::Armour(armour) => armour.name.eq(item_name_to_remove),
        });

        match index {
            Some(i) => {
                self.inventory.remove(i);
            }
            None => (),
        }
    }

    pub fn get_battle_gear(&self) -> &BattleGear {
        &self.battle_gear
    }

    pub fn equip_item<'b>(&'b mut self, item: &'a Item) {
        self.remove_item_from_inventory(item);
        let item_to_return_to_inventory = self.battle_gear.equip_item(item);
        if let Some(item_to_return) = item_to_return_to_inventory {
            self.add_item_to_inventory(item_to_return);
        }
    }

    pub fn unequip_item<'b>(&'b mut self, item: &'a Item) {
        self.add_item_to_inventory(item);
        self.battle_gear.unequip_item(item);
    }

    pub fn give_exp(&mut self, exp: u32) -> u32 {
        self.exp += exp;
        self.level_up()
    }

    pub fn exp_to_next_level(&self) -> u32 {
        (self.level + 1).pow(2)
    }

    pub fn level_up(&mut self) -> u32 {
        let mut levels_gained: u32 = 0;

        while self.exp >= self.exp_to_next_level() {
            self.exp -= self.exp_to_next_level();
            self.level += 1;
            self.strength += 1;
            levels_gained += 1;
        }

        levels_gained
    }
}
