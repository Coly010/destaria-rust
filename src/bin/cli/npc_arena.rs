use crate::{battle, output};
use destaria::game::player::{Player, NPC};
use destaria::game::system::cli::get_cli_input_with_prompt;

use inquire::{InquireError, Select};

const SELECT_NPC_COMMAND: &str = "1";
const LEAVE_NPC_ARENA: &str = "2";

pub fn load_npc_arena(mut player: &mut Player, mut npcs: &mut Vec<&NPC>) {
    print_npc_arena();
    npcs.sort_by(|a,b| a.level.cmp(&b.level));

    'npc_arena_loop: loop {
        print_npc_arena_options(npcs);
        let command = get_cli_input_with_prompt("> ");
        if command.eq(SELECT_NPC_COMMAND) {
            let npc: Result<&NPC, InquireError> =
                Select::new("Select an NPC", npcs.to_vec()).prompt();
            match npc {
                Ok(npc) => {
                    battle::battle(&mut player, npc);
                }
                Err(_) => (),
            }
        } else if command.eq(LEAVE_NPC_ARENA) {
            output::print_game_logo();
            break 'npc_arena_loop;
        }
    }
}

pub fn print_npc_arena() {
    output::print_game_logo();
}

pub fn print_npc_arena_options(npcs: &Vec<&NPC>) {
    println!(
        "\nYou enter the NPC Arena and see a bulletin board with a list of potential opponents: \n"
    );
    for npc in npcs {
        println!("{}", npc);
    }
    output::print_section_title("NPC Arena Options:");
    println!("[{}]: Select NPC to Battle", SELECT_NPC_COMMAND);
    println!("[{}]: Leave NPC Arena", LEAVE_NPC_ARENA);
}
