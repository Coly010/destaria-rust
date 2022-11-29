use destaria::game::battle::{Battle, BattleResult, BattleState, BattleTurn};
use destaria::game::player::{Player, NPC};
use destaria::game::system::cli::get_cli_input_with_prompt;

use super::output;
use colored::Colorize;

const BATTLE_OPTIONS_ATTACK: &str = "1";
const BATTLE_OPTIONS_AUTO_ATTACK: &str = "2";
const BATTLE_OPTIONS_FLEE: &str = "3";

pub fn battle(player: &mut Player, npc: &NPC) {
    output::print_game_logo();
    println!("Starting battle with {}", npc.name);

    let mut battle = Battle::new();
    battle.start_battle(&player, &npc);

    'attack_loop: loop {
        output::print_battle_options(&player, &npc, &battle);

        let command = get_cli_input_with_prompt("> ");

        output::print_game_logo();
        if command.eq(BATTLE_OPTIONS_ATTACK) {
            for _n in 1..=2 {
                let battle_result = do_battle_turn(player, &npc, &mut battle);

                if battle_result.won_or_lost {
                    process_exp_reward(battle_result, player);

                    break 'attack_loop;
                }
            }
        } else if command.eq(BATTLE_OPTIONS_AUTO_ATTACK) {
            while battle.is_active() {
                let battle_result = do_battle_turn(player, &npc, &mut battle);

                if battle_result.won_or_lost {
                    process_exp_reward(battle_result, player);

                    break 'attack_loop;
                }
            }
        } else if command.eq(BATTLE_OPTIONS_FLEE) {
            println!("You fled from the battle!");
            break 'attack_loop;
        }
    }
}

fn process_exp_reward(battle_result: BattleResult, player: &mut Player) {
    if let Some(exp_reward) = battle_result.exp_reward {
        let levels_gained = player.give_exp(exp_reward);
        if levels_gained > 0 {
            println!("You leveled up! You gained {} levels.", levels_gained);
            println!(
                "You are now level {}! Your 💪 is {}!",
                player.level, player.strength
            );
        }
    }
}

fn do_battle_turn(player: &Player, npc: &NPC, mut battle: &mut Battle) -> BattleResult {
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

fn is_battle_finished(battle: &mut Battle, player: &Player, npc: &NPC) -> BattleResult {
    match battle.check_battle_status() {
        BattleState::Won => {
            let exp_reward = calculate_exp_reward(&player, &npc);
            println!("\n\n{}", String::from("You won!").green().bold());
            println!(
                "{}",
                String::from(format!(
                    "You gained {} exp!",
                    calculate_exp_reward(&player, &npc)
                ))
                .yellow()
            );

            BattleResult {
                won_or_lost: true,
                exp_reward: Some(exp_reward),
            }
        }
        BattleState::Lost => {
            println!("\n\n{}", String::from("You lost!").red().bold());
            BattleResult {
                won_or_lost: true,
                exp_reward: None,
            }
        }
        _ => {
            battle.change_turn();
            BattleResult {
                won_or_lost: false,
                exp_reward: None,
            }
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
