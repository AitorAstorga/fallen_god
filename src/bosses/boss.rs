// src/boss.rs

use macroquad::prelude::*;
use crate::objects::objects::GameObject;
use crate::types::GamePhase;

pub struct Boss {
    pub base: GameObject,
    pub life: i32,
}

impl Boss {
    pub async fn new(pos: Vec2, life: i32, texture_path: &str) -> Self {
        let texture = load_texture(texture_path).await.unwrap();
        let base = GameObject::new_with_texture("boss", pos, vec2(96.0, 96.0), texture);
        Self { base, life }
    }
    
    pub fn draw(&self) {
        self.base.draw();
    }
    
    pub fn as_object(&self) -> &GameObject {
        &self.base
    }
}

pub fn check_boss_game_state(player_lives: i32, boss_life: i32) -> GamePhase {
    if player_lives <= 0 {
        GamePhase::Lose
    } else if boss_life <= 0 {
        GamePhase::Win
    } else {
        GamePhase::Boss
    }
}
