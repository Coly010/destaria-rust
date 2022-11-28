use destaria::game::item::Item;
use destaria::game::player::Player;
use destaria::game::system::cli::get_cli_input_with_prompt;

use colored::Colorize;
use inquire::{InquireError, Select};

use super::output;

pub fn load_inventory(player: &mut Player) {
    print_inventory(&player);

    'inventory_loop: loop {
        print_inventory_options(&player);
        let command = get_cli_input_with_prompt("> ");
        if command.eq("1") {
            let player_inventory = &player.inventory;
            let items = player_inventory.to_vec();
            let item_to_equip: Result<&Item, InquireError> = Select::new("Select an item to equip", items).prompt();
            match item_to_equip {
                Ok(item) => {
                    player.equip_item(item);
                    println!("Equipped {}!", item);
                    print_inventory(&player);
                },
                Err(_) => ()
            }
        } else if command.eq("2") {
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
        items.for_each(|item| {
            inventory.push_str(
                format!(
                    "{} ({})\n",
                    item.name(),
                    item.get_name_of_item_type().blue()
                )
                .as_str(),
            )
        });
        println!("{}", inventory);
    }
}

fn print_inventory_options(player: &Player) {
    let inv = &player.inventory;
    let items = inv.iter();

    println!("\n");
    println!("==============================");
    println!("\n");
    println!("Inventory Options:");

    if items.len() > 0 {
        println!("[1]: Equip Item");
    }
    println!("[2]: Leave");
}
