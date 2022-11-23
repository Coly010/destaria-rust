use destaria::game::item::{Item, Armour, ArmourType, Weapon};
use destaria::game::player::{NPC, Player};
use destaria::game::battle::{Battle, BattleState, BattleTurn};

fn main() {

    let sword = Item::Weapon(Weapon {
        name: String::from("Sword"),
        damage: 2
    });

    let helmut = Item::Armour(Armour {
        name: String::from("Helmut"),
        protection: 2,
        armour_type: ArmourType::Head
    });

    let mut player = Player::new();

    player.add_item_to_inventory(&sword);
    player.add_item_to_inventory(&helmut);


    let inv = player.get_inventory();
    for item in inv {
        println!("You have a {} in your inventory", item.name());
    }

    player.equip_item(&helmut);
    player.equip_item(&sword);

    let battle_gear = player.get_battle_gear();
    println!("You currently have equipped:");

    if let Some(item) = battle_gear.head {
        println!("Head: {}", item.name());
    }
    if let Some(item) = battle_gear.body {
        println!("Body: {}", item.name());
    }
    if let Some(item) = battle_gear.legs {
        println!("Legs: {}", item.name());
    }
    if let Some(item) = battle_gear.hands {
        println!("Hands: {}", item.name());
    }
    if let Some(item) = battle_gear.feet {
        println!("Feet: {}", item.name());
    }
    if let Some(item) = battle_gear.weapon {
        println!("Weapon: {}", item.name());
    }
    println!("Your total protection is {}, and your damage is {}", battle_gear.calculate_protection(), battle_gear.calculate_damage());


    let mut npc = NPC::new(String::from("Troll"));
    npc.equip_item(&sword);

    println!("A wild {} has appeared! It has {} damage!", npc.name, npc.get_battle_gear().calculate_damage());

    let mut battle = Battle::new();
    battle.start_battle(&player, &npc);

    println!("You started battling the {}! You have {}hp and the {} has {}hp!", npc.name, battle.player_hp, npc.name, battle.npc_hp);

    while battle.is_active() {
        let attack_result = battle.attack();
        match battle.get_turn() {
            BattleTurn::Player => {
                if let Some(weapon) = player.get_battle_gear().weapon {
                    println!("You attacked the {} with your {}, dealing {} damage! They have {}hp left", npc.name, weapon.name(), attack_result.damage_dealt, attack_result.hp_remaining);
                }
            },
            BattleTurn::NPC => {
                if let Some(weapon) = npc.get_battle_gear().weapon {
                    println!("The {} attacked you with their {}, dealing {} damage! You have {}hp left", npc.name, weapon.name(), attack_result.damage_dealt, attack_result.hp_remaining);
                }
            }
        }

        match battle.check_battle_status() {
            BattleState::Won => {
                println!("You won!");
            },
            BattleState::Lost => {
                println!("You lost!");
            },
            _ => battle.change_turn()
        }
    }

}