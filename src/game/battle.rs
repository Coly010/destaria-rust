use super::player::{Player, NPC};

pub enum BattleState {
    NotStarted,
    Active,
    Won,
    Lost,
}

pub enum BattleTurn {
    Player,
    NPC,
}

pub struct BattleResult {
    pub won_or_lost: bool,
    pub exp_reward: Option<u32>,
    pub money_reward: Option<u32>,
}

pub struct AttackResult {
    pub damage_dealt: u32,
    pub hp_remaining: u32,
}

pub struct Battle<'a> {
    pub player_hp: u32,
    pub npc_hp: u32,
    player: Option<&'a Player<'a>>,
    npc: Option<&'a NPC<'a>>,
    state: BattleState,
    turn: BattleTurn,
}

impl<'a> Battle<'a> {
    pub fn new() -> Battle<'a> {
        Battle {
            player: None,
            npc: None,
            player_hp: 0,
            npc_hp: 0,
            state: BattleState::NotStarted,
            turn: BattleTurn::Player,
        }
    }

    pub fn get_turn(&self) -> &BattleTurn {
        &self.turn
    }

    pub fn get_state(&self) -> &BattleState {
        &self.state
    }

    pub fn is_active(&self) -> bool {
        match self.get_state() {
            BattleState::Active => true,
            _ => false,
        }
    }

    pub fn start_battle(&mut self, player: &'a Player, npc: &'a NPC) {
        self.player = Some(player);
        self.npc = Some(npc);
        self.player_hp = player.level * 100;
        self.npc_hp = npc.level * 100;
        self.state = BattleState::Active;
    }

    pub fn attack(&mut self) -> AttackResult {
        let mut damage_dealt: u32 = 0;
        let mut hp_remaining: u32 = 0;

        if let Some(player) = self.player {
            if let Some(npc) = self.npc {
                match self.turn {
                    BattleTurn::Player => {
                        damage_dealt = match (player.get_battle_gear().calculate_damage()
                            + player.strength)
                            .checked_sub(npc.get_battle_gear().calculate_protection())
                        {
                            Some(v) => v,
                            None => 1,
                        };

                        self.npc_hp = match self.npc_hp.checked_sub(damage_dealt) {
                            Some(v) => v,
                            None => 0,
                        };

                        hp_remaining = self.npc_hp;
                    }
                    BattleTurn::NPC => {
                        damage_dealt = match (npc.get_battle_gear().calculate_damage()
                            + npc.strength)
                            .checked_sub(player.get_battle_gear().calculate_protection())
                        {
                            Some(v) => {
                                if v == 0 {
                                    1
                                } else {
                                    v
                                }
                            }
                            None => 1,
                        };

                        self.player_hp = match self.player_hp.checked_sub(damage_dealt) {
                            Some(v) => v,
                            None => 0,
                        };

                        hp_remaining = self.player_hp;
                    }
                }
            }
        }

        AttackResult {
            damage_dealt,
            hp_remaining,
        }
    }

    pub fn change_turn(&mut self) {
        self.turn = match self.turn {
            BattleTurn::Player => BattleTurn::NPC,
            BattleTurn::NPC => BattleTurn::Player,
        }
    }

    pub fn check_battle_status(&mut self) -> &BattleState {
        if self.npc_hp == 0 {
            self.state = BattleState::Won;
        } else if self.player_hp == 0 {
            self.state = BattleState::Lost;
        }

        &self.state
    }
}
