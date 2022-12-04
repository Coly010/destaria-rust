use destaria::game::item::Item;
use destaria::game::player::Player;
use destaria::game::system::cli::get_cli_input_with_prompt;

use super::output;

use crate::output::print_section_title;
use colored::Colorize;
use inquire::{InquireError, Select};

const EQUIPMENT_OPTIONS_UNEQUIP_ITEM: &str = "1";
const EQUIPMENT_OPTIONS_QUIT: &str = "2";

pub fn load_equipment(player: &mut Player) {
    print_equipment(&player);

    'equipment_loop: loop {
        print_equipment_options(&player);
        let command = get_cli_input_with_prompt("> ");
        if command.eq(EQUIPMENT_OPTIONS_UNEQUIP_ITEM) {
            let battle_gear = &player.battle_gear;
            let items: Vec<&Item> = battle_gear.get_all_items_equipped_as_vec();
            let item_to_unequip: Result<&Item, InquireError> =
                Select::new("Select an item to equip", items).prompt();
            match item_to_unequip {
                Ok(item) => {
                    player.unequip_item(item);
                    print_equipment(&player);
                    println!("\n");
                    println!("Successfully unequipped {}!", item);
                }
                Err(_) => (),
            }
        } else if command.eq(EQUIPMENT_OPTIONS_QUIT) {
            output::print_game_logo();
            break 'equipment_loop;
        }
    }
}

pub fn print_equipment_options(player: &Player) {
    print_section_title("Equipment Options:");

    let has_equipped_items = player.get_battle_gear().has_items_equipped();
    if has_equipped_items {
        println!("[{}]: Unequip Item", EQUIPMENT_OPTIONS_UNEQUIP_ITEM);
    }
    println!("[{}]: Leave", EQUIPMENT_OPTIONS_QUIT);
}

pub fn print_equipment(player: &Player) {
    output::print_game_logo();

    let battle_gear = player.get_battle_gear();
    println!("You currently have equipped:");

    if let Some(item) = battle_gear.head {
        println!(
            "{}",
            format!(
                "{} ({}, {})\n",
                item.name(),
                item.get_name_of_item_type().italic().blue(),
                format!("{} protection", item.get_item_protection_or_damage())
                    .italic()
                    .blue()
            )
        );
    }
    if let Some(item) = battle_gear.body {
        println!(
            "{}",
            format!(
                "{} ({}, {})\n",
                item.name(),
                item.get_name_of_item_type().italic().blue(),
                format!("{} protection", item.get_item_protection_or_damage())
                    .italic()
                    .blue()
            )
        );
    }
    if let Some(item) = battle_gear.legs {
        println!(
            "{}",
            format!(
                "{} ({}, {})\n",
                item.name(),
                item.get_name_of_item_type().italic().blue(),
                format!("{} protection", item.get_item_protection_or_damage())
                    .italic()
                    .blue()
            )
        );
    }
    if let Some(item) = battle_gear.hands {
        println!(
            "{}",
            format!(
                "{} ({}, {})\n",
                item.name(),
                item.get_name_of_item_type().italic().blue(),
                format!("{} protection", item.get_item_protection_or_damage())
                    .italic()
                    .blue()
            )
        );
    }
    if let Some(item) = battle_gear.feet {
        println!(
            "{}",
            format!(
                "{} ({}, {})\n",
                item.name(),
                item.get_name_of_item_type().italic().blue(),
                format!("{} protection", item.get_item_protection_or_damage())
                    .italic()
                    .blue()
            )
        );
    }
    if let Some(item) = battle_gear.weapon {
        println!(
            "{}",
            format!(
                "{} ({}, {})\n",
                item.name(),
                item.get_name_of_item_type().italic().bright_red(),
                format!("{} dmg", item.get_item_protection_or_damage())
                    .italic()
                    .bright_red()
            )
        );
    }
    println!(
        "Your {} is {}, and your {} is {}",
        "total protection".italic().blue(),
        battle_gear
            .calculate_protection()
            .to_string()
            .italic()
            .blue(),
        "damage".italic().bright_red(),
        battle_gear
            .calculate_damage()
            .to_string()
            .italic()
            .bright_red()
    );
}
