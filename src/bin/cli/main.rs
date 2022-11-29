use destaria::game::content::base::{get_item_armour_keys, get_item_weapon_keys, get_items, get_npc_keys, get_npcs};
use destaria::game::item::{Armour, ArmourType, Item, Weapon};
use destaria::game::player::{Player, NPC};
use destaria::game::system::cli::get_cli_input_with_prompt;

mod battle;
mod equipment;
mod inventory;
mod output;

fn main() {
    let ITEMS = get_items();
    let ITEM_ARMOUR_KEYS = get_item_armour_keys();
    let ITEM_WEAPON_KEYS = get_item_weapon_keys();
    let NPCS = get_npcs(&ITEMS);
    let NPC_KEYS = get_npc_keys();

    let mut player = Player::new();

    let mut initial_items_log = String::new();

    let weapon_key: &str = ITEM_WEAPON_KEYS[fastrand::usize(..ITEM_WEAPON_KEYS.len())];
    if let Some(weapon) = ITEMS.get(weapon_key) {
        player.add_item_to_inventory(weapon);
        initial_items_log.push_str(format!("You've been given {}!\n", weapon).as_str());
    }

    let armour_key: &str = ITEM_ARMOUR_KEYS[fastrand::usize(..ITEM_ARMOUR_KEYS.len())];
    if let Some(armour) = ITEMS.get(armour_key) {
        player.add_item_to_inventory(armour);
        initial_items_log.push_str(format!("You've been given {}!\n", armour).as_str());
    }

    let mut npc: Option<&NPC> = None;
    let npc_key: &str = NPC_KEYS[fastrand::usize(..NPC_KEYS.len())];
    if let Some(selected_npc) = NPCS.get(npc_key) {
        npc = Some(selected_npc);
    }

    output::print_game_logo();

    println!("{}", initial_items_log);

    'game_loop: loop {
        output::print_game_options(&player);

        let command = get_cli_input_with_prompt("> ");
        if command.eq("1") {
            if let Some(npc) = npc {
                battle::battle(&mut player, &npc)
            }
        } else if command.eq("2") {
            equipment::load_equipment(&mut player);
        } else if command.eq("3") {
            inventory::load_inventory(&mut player);
        } else if command.to_lowercase().eq("q") || command.to_lowercase().eq("quit") {
            println!("Quitting game...");
            break 'game_loop;
        }
    }
}
