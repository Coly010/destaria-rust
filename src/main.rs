use destaria::game::battle::{Battle, BattleState, BattleTurn};
use destaria::game::item::{Armour, ArmourType, Item, Weapon};
use destaria::game::player::{Player, NPC};
use destaria::game::system::cli::get_cli_input_with_prompt;

use colored::Colorize;

fn main() {
    let sword = Item::Weapon(Weapon {
        name: String::from("Sword"),
        damage: 2,
    });

    let helmut = Item::Armour(Armour {
        name: String::from("Helmut"),
        protection: 2,
        armour_type: ArmourType::Head,
    });

    let mut player = Player::new();

    player.add_item_to_inventory(&sword);
    player.add_item_to_inventory(&helmut);

    player.equip_item(&helmut);
    player.equip_item(&sword);

    let mut npc = NPC::new(String::from("Troll"));
    npc.equip_item(&sword);

    print_game_logo();

    'game_loop: loop {
        print_game_options();

        let command = get_cli_input_with_prompt("> ");
        if command.eq("1") {
            battle(&mut player, &mut npc)
        } else if command.eq("2") {
            print_equipment(&player);
        } else if command.eq("3") {
            print_inventory(&player);
        } else if command.to_lowercase().eq("q") || command.to_lowercase().eq("quit") {
            println!("Quitting game...");
            break 'game_loop;
        }
    }
}

fn battle(player: &Player, npc: &NPC) {
    print_game_logo();
    println!("Starting battle with {}", npc.name);

    let mut battle = Battle::new();
    battle.start_battle(&player, &npc);

    'attack_loop: loop {
        print_battle_options(&npc, &battle);

        let command = get_cli_input_with_prompt("> ");

        print_game_logo();
        if command.eq("1") {
            for _n in 1..=2 {
                if do_battle_turn(&player, &npc, &mut battle) {
                    break 'attack_loop;
                }
            }
        } else if command.eq("2") {
            while battle.is_active() {
                if do_battle_turn(&player, &npc, &mut battle) {
                    break 'attack_loop;
                }
            }
        } else if command.eq("3") {
            println!("You fled from the battle!");
            break 'attack_loop;
        }
    }
}

fn do_battle_turn(player: &Player, npc: &NPC, mut battle: &mut Battle) -> bool {
    perform_attack(&player, &npc, &mut battle);

    is_battle_finished(&mut battle)
}

fn perform_attack(player: &Player, npc: &NPC, battle: &mut Battle) {
    let attack_result = battle.attack();
    match battle.get_turn() {
        BattleTurn::Player => {
            if let Some(weapon) = player.get_battle_gear().weapon {
                println!(
                    "{}",
                    String::from(format!(
                        "You attacked the {} with your {}, dealing {} damage!",
                        npc.name,
                        weapon.name(),
                        attack_result.damage_dealt
                    ))
                    .blue()
                )
            }
        }
        BattleTurn::NPC => {
            if let Some(weapon) = npc.get_battle_gear().weapon {
                println!(
                    "{}",
                    String::from(format!(
                        "The {} attacked you with their {}, dealing {} damage!",
                        npc.name,
                        weapon.name(),
                        attack_result.damage_dealt
                    ))
                    .purple()
                );
            }
        }
    }
}

fn is_battle_finished(battle: &mut Battle) -> bool {
    match battle.check_battle_status() {
        BattleState::Won => {
            println!("\n\n{}", String::from("You won!").green().bold());
            true
        }
        BattleState::Lost => {
            println!("\n\n{}", String::from("You lost!").red().bold());
            true
        }
        _ => {
            battle.change_turn();
            false
        }
    }
}

fn print_game_options() {
    println!("\n");
    println!("==============================");
    println!("Game Options:");
    println!("[1]: Battle");
    println!("[2]: Check Equipment");
    println!("[3]: Check Inventory");
    println!("[Q]: Quit");
}

fn print_battle_options(npc: &NPC, battle: &Battle) {
    println!("\n\n");
    println!("==============================");
    println!("Battle Stats");
    println!("==============================");
    println!(
        "You: {}hp \t {}: {}hp",
        battle.player_hp, npc.name, battle.npc_hp
    );
    println!("\n");
    println!("Battle Options:");
    println!("[1]: Attack");
    println!("[2]: Auto Attack (Note: You will not be able to flee!)");
    println!("[3]: Flee");
}

fn print_inventory(player: &Player) {
    print_game_logo();

    let inv = player.get_inventory();
    let items = inv.iter();

    if items.len() == 0 {
        println!("You have no items in your inventory!");
    } else {
        items.for_each(|item| println!("You have a {} in your inventory", item.name()));
    }
}

fn print_equipment(player: &Player) {
    print_game_logo();

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
    println!(
        "Your total protection is {}, and your damage is {}",
        battle_gear.calculate_protection(),
        battle_gear.calculate_damage()
    );
}

fn print_game_logo() {
    print!("{}[2J", 27 as char);
    println!("████████▄     ▄████████    ▄████████     ███        ▄████████    ▄████████  ▄█     ▄████████");
    println!("███   ▀███   ███    ███   ███    ███ ▀█████████▄   ███    ███   ███    ███ ███    ███    ███");
    println!("███    ███   ███    █▀    ███    █▀     ▀███▀▀██   ███    ███   ███    ███ ███▌   ███    ███");
    println!("███    ███  ▄███▄▄▄       ███            ███   ▀   ███    ███  ▄███▄▄▄▄██▀ ███▌   ███    ███");
    println!("███    ███ ▀▀███▀▀▀     ▀███████████     ███     ▀███████████ ▀▀███▀▀▀▀▀   ███▌ ▀███████████");
    println!("███    ███   ███    █▄           ███     ███       ███    ███ ▀███████████ ███    ███    ███");
    println!("███   ▄███   ███    ███    ▄█    ███     ███       ███    ███   ███    ███ ███    ███    ███");
    println!("████████▀    ██████████  ▄████████▀     ▄████▀     ███    █▀    ███    ███ █▀     ███    █▀");
    println!("                                                                ███    ███");

    println!("{}", String::from("A CLI Turn-Based Battle Game!").green());
    println!("\n");
    println!("==============================");
    println!("\n");
}
