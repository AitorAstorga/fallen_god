// src/bosses/boss3_b.rs
use macroquad::texture::load_texture;
use crate::{assets::BOSS3_B_IMAGE, types::GamePhase};
use super::boss3::{run_boss_battle, BossConfig};

pub async fn boss3_b() -> GamePhase {
    let config = BossConfig {
        life: 20,
        speed: 0.0115,
        mines: 25,
        boss_texture: load_texture(BOSS3_B_IMAGE).await.unwrap()
    };
    run_boss_battle(config).await
}
