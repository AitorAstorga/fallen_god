// src/bosses/boss1_b.rs
use macroquad::texture::load_texture;
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
        boss_texture: load_texture(BOSS1_B_IMAGE).await.unwrap(),
        boss_shot_texture: load_texture(BOSS1_SHOT).await.unwrap()
    };
    run_boss_battle(config).await
}
