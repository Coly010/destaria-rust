use destaria::game::item::Item;
use destaria::game::player::Player;
use destaria::game::system::cli::get_cli_input_with_prompt;

use crate::output::print_section_title;
use colored::Colorize;
use inquire::{InquireError, Select};

use super::output;

const INVENTORY_OPTIONS_EQUIP_ITEM: &str = "1";
const INVENTORY_OPTIONS_QUIT: &str = "2";

pub fn load_inventory(player: &mut Player) {
    print_inventory(&player);

    'inventory_loop: loop {
        print_inventory_options(&player);
        let command = get_cli_input_with_prompt("> ");
        if command.eq(INVENTORY_OPTIONS_EQUIP_ITEM) {
            let player_inventory = &player.inventory;
            let items = player_inventory.to_vec();
            let item_to_equip: Result<&Item, InquireError> =
                Select::new("Select an item to equip", items).prompt();
            match item_to_equip {
                Ok(item) => {
                    player.equip_item(item);
                    print_inventory(&player);
                    println!("\n");
                    println!("Successfully equipped {}!", item);
                }
                Err(_) => (),
            }
        } else if command.eq(INVENTORY_OPTIONS_QUIT) {
            output::print_game_logo();
            break 'inventory_loop;
        }
    }
}

fn print_inventory(player: &Player) {
    output::print_game_logo();

    let inv = &player.inventory;
    let items = inv.iter();

    if items.len() == 0 {
        println!("You have no items in your inventory!");
    } else {
        println!("Items in your inventory:");
        let mut inventory = String::new();
        items.for_each(|item| match item {
            Item::Armour(_armour) => inventory.push_str(
                format!(
                    "{} ({}, {})\n",
                    item.name(),
                    item.get_name_of_item_type().italic().blue(),
                    format!("{} protection", item.get_item_protection_or_damage())
                        .italic()
                        .blue()
                )
                .as_str(),
            ),
            Item::Weapon(_weapon) => inventory.push_str(
                format!(
                    "{} ({}, {})\n",
                    item.name(),
                    item.get_name_of_item_type().italic().bright_red(),
                    format!("{} dmg", item.get_item_protection_or_damage())
                        .italic()
                        .bright_red()
                )
                .as_str(),
            ),
        });
        println!("{}", inventory);
    }
}

fn print_inventory_options(player: &Player) {
    let inv = &player.inventory;
    let items = inv.iter();

    print_section_title("Inventory Options:");

    if items.len() > 0 {
        println!("[{}]: Equip Item", INVENTORY_OPTIONS_EQUIP_ITEM);
    }
    println!("[{}]: Leave", INVENTORY_OPTIONS_QUIT);
}
