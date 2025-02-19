// src/bosses/boss4_b.rs
use crate::types::GamePhase;
use super::boss4::{run_boss_battle, BossConfig};

pub async fn boss4_b() -> GamePhase {
    let config = BossConfig {
        life: 20,
        bullet_speed_multiplier: 5.0,
        mines: 12
    };
    run_boss_battle(config).await
}