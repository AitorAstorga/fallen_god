// src/types.rs

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum GamePhase {
    Boss,
    Lose,
    Win,
}
