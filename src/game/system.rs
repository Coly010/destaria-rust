use std::thread;
use std::time::Duration;

pub mod cli;

pub enum GameState {
    Started,
    Paused,
    NotStarted,
}

pub fn fixed_game_loop(fps: u64, game_logic: fn()) {
    loop {
        game_logic();
        thread::sleep(Duration::from_millis(1000 / fps));
    }
}
