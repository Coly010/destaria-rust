use destaria::game::battle::{Battle, BattleState, BattleTurn};
use destaria::game::player::{Player, NPC};
use destaria::game::system::cli::get_cli_input_with_prompt;

use super::output;
use colored::Colorize;

pub fn battle(player: &Player, npc: &NPC) {
    output::print_game_logo();
    println!("Starting battle with {}", npc.name);

    let mut battle = Battle::new();
    battle.start_battle(&player, &npc);

    'attack_loop: loop {
        output::print_battle_options(&player, &npc, &battle);

        let command = get_cli_input_with_prompt("> ");

        output::print_game_logo();
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

    is_battle_finished(&mut battle, &player, &npc)
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

fn is_battle_finished(battle: &mut Battle, player: &Player, npc: &NPC) -> bool {
    match battle.check_battle_status() {
        BattleState::Won => {
            println!("\n\n{}", String::from("You won!").green().bold());
            println!(
                "{}",
                String::from(format!(
                    "You gained {} exp!",
                    calculate_exp_reward(&player, &npc)
                ))
                .yellow()
            );
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

fn calculate_exp_reward(player: &Player, npc: &NPC) -> u32 {
    let mut reward: u32 = 1;
    if player.strength < npc.strength {
        reward += 2;
    } else {
        reward += 1;
    }

    if player.level < npc.level {
        let difference_in_level = npc.level - player.level;
        reward = (reward * (difference_in_level + 1)) / 2;
    } else {
        reward += 1;
    }

    reward
}
