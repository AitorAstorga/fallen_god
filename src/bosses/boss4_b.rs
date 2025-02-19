// src/bosses/boss4_b.rs
use crate::{assets::{BOSS4_B_LEFT_IMAGE, BOSS4_B_RIGHT_IMAGE, MAP_BOSS4_B}, types::GamePhase};
use super::boss4::{run_boss_battle, BossConfig};

pub async fn boss4_b() -> GamePhase {
    let config = BossConfig {
        life: 20,
        bullet_speed_multiplier: 5.0,
        mines: 12,
        boss_image_left_path: BOSS4_B_LEFT_IMAGE,
        boss_image_right_path: BOSS4_B_RIGHT_IMAGE,
        map_path: MAP_BOSS4_B
    };
    run_boss_battle(config).await
}