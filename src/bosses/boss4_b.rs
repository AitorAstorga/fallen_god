// src/bosses/boss4_b.rs
use macroquad::texture::load_texture;
use crate::{assets::{BOSS4_B_LEFT_IMAGE, BOSS4_B_RIGHT_IMAGE, MAP_BOSS4_B}, types::GamePhase};
use super::boss4::{run_boss_battle, BossConfig};

pub async fn boss4_b() -> GamePhase {
    let config = BossConfig {
        life: 20,
        bullet_speed_multiplier: 5.0,
        mines: 12,
        boss_texture_left: load_texture(BOSS4_B_LEFT_IMAGE).await.unwrap(),
        boss_texture_right: load_texture(BOSS4_B_RIGHT_IMAGE).await.unwrap(),
        map_texture: load_texture(MAP_BOSS4_B).await.unwrap()
    };
    run_boss_battle(config).await
}