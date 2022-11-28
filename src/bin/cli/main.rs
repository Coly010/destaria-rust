use destaria::game::item::{Armour, ArmourType, Item, Weapon};
use destaria::game::player::{Player, NPC};
use destaria::game::system::cli::get_cli_input_with_prompt;

mod battle;
mod equipment;
mod inventory;
mod output;

fn main() {
    let sword = Item::Weapon(Weapon {
        name: String::from("Sword"),
        damage: 2,
    });

    let axe = Item::Weapon(Weapon {
        name: String::from("Axe"),
        damage: 4,
    });

    let helmut = Item::Armour(Armour {
        name: String::from("Helmut"),
        protection: 2,
        armour_type: ArmourType::Head,
    });

    let chestplate = Item::Armour(Armour {
        name: String::from("Chestplate"),
        protection: 4,
        armour_type: ArmourType::Body,
    });

    let mut player = Player::new();

    player.add_item_to_inventory(&sword);
    player.add_item_to_inventory(&helmut);
    player.add_item_to_inventory(&chestplate);
    player.add_item_to_inventory(&axe);

    player.equip_item(&helmut);
    player.equip_item(&sword);

    let mut npc = NPC::new(String::from("Troll"));
    npc.equip_item(&sword);

    output::print_game_logo();

    'game_loop: loop {
        output::print_game_options(&player);

        let command = get_cli_input_with_prompt("> ");
        if command.eq("1") {
            battle::battle(&mut player, &mut npc)
        } else if command.eq("2") {
            equipment::print_equipment(&player);
        } else if command.eq("3") {
            inventory::load_inventory(&mut player);
        } else if command.to_lowercase().eq("q") || command.to_lowercase().eq("quit") {
            println!("Quitting game...");
            break 'game_loop;
        }
    }
}
