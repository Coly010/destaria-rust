use destaria::game::player::Player;

use super::output;

use colored::Colorize;

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
                format!("{} protection", item.get_item_protection_or_damage()).italic().blue()
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
                format!("{} protection", item.get_item_protection_or_damage()).italic().blue()
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
                format!("{} protection", item.get_item_protection_or_damage()).italic().blue()
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
                format!("{} protection", item.get_item_protection_or_damage()).italic().blue()
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
                format!("{} protection", item.get_item_protection_or_damage()).italic().blue()
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
                format!("{} dmg", item.get_item_protection_or_damage()).italic().bright_red()
            )
        );
    }
    println!(
        "Your {} is {}, and your {} is {}",
        "total protection".italic().blue(),
        battle_gear.calculate_protection().to_string().italic().blue(),
        "damage".italic().bright_red(),
        battle_gear.calculate_damage().to_string().italic().bright_red()
    );
}
