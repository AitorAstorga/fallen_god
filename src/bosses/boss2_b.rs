// src/bosses/boss2_b.rs
use macroquad::prelude::*;
use crate::{
    assets::*,
    types::GamePhase,
};
use super::boss2::{run_boss_battle, BossConfig};

/// Example implementation of boss2_b with two bosses.
pub async fn boss2_b() -> GamePhase {
    let config = BossConfig {
        collision_limit: 5,
        boss_a_life: 20,
        // Boss A configuration.
        boss_a_initial_position: vec2(500.0, 225.0),
        boss_a_shot_texture: BOSS2_SHOT,
        boss_a_normal_texture: BOSS2_IMAGE,
        boss_a_speed: 4.0,
        // Boss B configuration.
        boss_b_life: Some(20),
        boss_b_initial_position: Some(vec2(500.0, 125.0)),
        boss_b_shot_texture: Some(BOSS2_B_SHOT),
        boss_b_normal_texture: Some(BOSS2_B_IMAGE),
        boss_b_speed: Some(5.0),
    };
    run_boss_battle(config).await
}
