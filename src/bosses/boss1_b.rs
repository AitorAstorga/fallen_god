// src/bosses/boss1_b.rs
use crate::{
    assets::*,
    types::GamePhase,
};
use crate::bosses::boss1::BossConfig;
use crate::bosses::boss1::run_boss_battle;

pub async fn boss1_b() -> GamePhase {
    let config = BossConfig {
        shoot_interval: 0.7,
        bullet_speed: 300.0,
        shot_precision: 1.0,
        life: 20,
        extra_bullet: true,
        map_texture_path: MAP_BOSS1,
        boss_image_path: BOSS1_B_IMAGE,
        boss_shot_sound: BOSS1_SHOT,
    };
    run_boss_battle(config).await
}
