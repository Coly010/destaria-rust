use destaria::game::battle::Battle;
use destaria::game::player::{Player, NPC};
use std::ops::{Div, Mul};

use colored::Colorize;

pub fn print_player_exp_bar(player: &Player) {
    let completion_percentage: f32 = (player.exp as f32).div(player.exp_to_next_level() as f32);
    let number_of_bars: f32 = 10.0;
    let number_of_complete_bars: u32 = number_of_bars.mul(completion_percentage) as u32;

    let mut complete_bars = String::new();
    for _n in 1..=number_of_complete_bars {
        complete_bars.push_str("#");
    }
    let mut remaining_bars = String::new();
    for _n in 1..=(number_of_bars as u32 - number_of_complete_bars) {
        remaining_bars.push_str("#");
    }
    println!("{}{}", complete_bars.blue(), remaining_bars.red());
}

pub fn print_game_options(player: &Player) {
    println!("\n");
    println!("==============================");
    println!("\n");
    println!("{}", String::from("Your Stats:").bold().yellow());
    println!("Level {} \t💪 {}", player.level, player.strength);
    println!("{}/{} exp", player.exp, player.exp_to_next_level());
    print_player_exp_bar(&player);
    println!("\n");
    println!("==============================");
    println!("Game Options:");
    println!("[1]: Battle");
    println!("[2]: Check Equipment");
    println!("[3]: Check Inventory");
    println!("[Q]: Quit");
}

pub fn print_battle_options(player: &Player, npc: &NPC, battle: &Battle) {
    println!("\n\n");
    println!("==============================");
    println!("Battle Stats");
    println!("==============================");
    println!(
        "You: {}hp \t {}: {}hp",
        battle.player_hp, npc.name, battle.npc_hp
    );
    println!(
        "You: 💪 {}  \t {}: 💪 {}",
        player.strength, npc.name, npc.strength
    );
    println!("You: {}/{} exp", player.exp, player.exp_to_next_level());
    print_player_exp_bar(&player);
    println!("\n");
    println!("Battle Options:");
    println!("[1]: Attack");
    println!("[2]: Auto Attack (Note: You will not be able to flee!)");
    println!("[3]: Flee");
}

pub fn print_game_logo() {
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
