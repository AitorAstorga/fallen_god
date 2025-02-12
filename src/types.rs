// src/types.rs

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum GamePhase {
    Boss1,
    Boss1b,
    Boss2,
    Boss2b,
    Boss3,
    Boss3b,
    Boss4,
    Boss4b,
    Lose,
    Win,
}
