use std::collections::HashMap;

use crate::game::item::{Armour, ArmourType, Item, Weapon};
use crate::game::player::NPC;

pub fn get_items<'a>() -> HashMap<&'a str, Item> {
    let mut items: HashMap<&str, Item> = HashMap::new();

    let teeth = Item::Weapon(Weapon {
        name: String::from("Teeth"),
        damage: 1,
    });

    let fangs = Item::Weapon(Weapon {
        name: String::from("Fangs"),
        damage: 2,
    });

    let claws = Item::Weapon(Weapon {
        name: String::from("Claws"),
        damage: 2,
    });

    let club = Item::Weapon(Weapon {
        name: String::from("Club"),
        damage: 4,
    });

    let dagger = Item::Weapon(Weapon {
        name: String::from("Dagger"),
        damage: 6,
    });

    let sword = Item::Weapon(Weapon {
        name: String::from("Sword"),
        damage: 10,
    });

    let leather_helm = Item::Armour(Armour {
        name: String::from("Leather Helm"),
        protection: 1,
        armour_type: ArmourType::Head,
    });

    let leather_tunic = Item::Armour(Armour {
        name: String::from("Leather Tunic"),
        protection: 2,
        armour_type: ArmourType::Body,
    });

    let leather_boots = Item::Armour(Armour {
        name: String::from("Leather Boots"),
        protection: 1,
        armour_type: ArmourType::Feet,
    });

    let leather_bracers = Item::Armour(Armour {
        name: String::from("Leather Bracers"),
        protection: 1,
        armour_type: ArmourType::Hands,
    });

    let iron_helm = Item::Armour(Armour {
        name: String::from("Iron Helm"),
        protection: 3,
        armour_type: ArmourType::Head,
    });

    let chain_mail = Item::Armour(Armour {
        name: String::from("Chain Mail"),
        protection: 4,
        armour_type: ArmourType::Body,
    });

    let iron_cuisses = Item::Armour(Armour {
        name: String::from("Iron Cuisses"),
        protection: 3,
        armour_type: ArmourType::Legs,
    });

    let iron_boots = Item::Armour(Armour {
        name: String::from("Iron Boots"),
        protection: 2,
        armour_type: ArmourType::Feet,
    });

    let iron_gauntlets = Item::Armour(Armour {
        name: String::from("Iron Gauntlets"),
        protection: 2,
        armour_type: ArmourType::Hands,
    });

    items.insert("teeth", teeth);
    items.insert("fangs", fangs);
    items.insert("claws", claws);
    items.insert("club", club);
    items.insert("dagger", dagger);
    items.insert("sword", sword);

    items.insert("leather_helm", leather_helm);
    items.insert("leather_tunic", leather_tunic);
    items.insert("leather_boots", leather_boots);
    items.insert("leather_bracers", leather_bracers);
    items.insert("iron_helm", iron_helm);
    items.insert("chain_mail", chain_mail);
    items.insert("iron_cuisses", iron_cuisses);
    items.insert("iron_boots", iron_boots);
    items.insert("iron_gauntlets", iron_gauntlets);

    items
}

pub fn get_item_weapon_keys<'a>() -> Vec<&'a str> {
    vec!["teeth", "fangs", "claws", "club", "dagger", "sword"]
}

pub fn get_item_armour_keys<'a>() -> Vec<&'a str> {
    vec![
        "leather_helm",
        "leather_tunic",
        "leather_boots",
        "leather_bracers",
        "iron_helm",
        "chain_mail",
        "iron_cuisses",
        "iron_boots",
        "iron_gauntlets",
    ]
}

pub fn get_npcs<'a>(items: &'a HashMap<&'a str, Item>) -> HashMap<&'a str, NPC<'a>> {
    let mut npcs: HashMap<&str, NPC> = HashMap::new();

    let mut rat = NPC::new(String::from("Rat"));
    if let Some(item) = items.get("teeth") {
        rat.equip_item(item);
    }

    let mut spider = NPC::new(String::from("Spider"));
    spider.level = 2;
    spider.strength = 3;
    if let Some(item) = items.get("fangs") {
        spider.equip_item(item);
    }

    let mut bear = NPC::new(String::from("Bear"));
    bear.level = 5;
    bear.strength = 8;
    if let Some(item) = items.get("claws") {
        bear.equip_item(item);
    }

    let mut troll = NPC::new(String::from("Troll"));
    troll.level = 7;
    troll.strength = 10;
    if let Some(item) = items.get("club") {
        troll.equip_item(item);
    }

    let mut goblin = NPC::new(String::from("Goblin"));
    goblin.level = 10;
    goblin.strength = 12;

    if let Some(item) = items.get("dagger") {
        goblin.equip_item(item);
    }
    if let Some(item) = items.get("leather_helm") {
        goblin.equip_item(item);
    }
    if let Some(item) = items.get("leather_tunic") {
        goblin.equip_item(item);
    }

    npcs.insert("rat", rat);
    npcs.insert("spider", spider);
    npcs.insert("bear", bear);
    npcs.insert("troll", troll);
    npcs.insert("goblin", goblin);

    npcs
}

pub fn get_npc_keys<'a>() -> Vec<&'a str> {
    vec!["rat", "spider", "bear", "troll", "goblin"]
}
