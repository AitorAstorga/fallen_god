// src/bosses/boss3_b.rs
use crate::types::GamePhase;
use super::boss3::{run_boss_battle, BossConfig};

pub async fn boss3_b() -> GamePhase {
    let config = BossConfig {
        life: 20,
        speed: 0.0115,
        mines: 25
    };
    run_boss_battle(config).await
}
