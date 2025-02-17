// src/objects/bullet.rs
use macroquad::prelude::*;

use super::objects::{GameObject, Shape};

pub struct Bullet {
    pub base: GameObject,
    pub vel: Vec2,
    pub active: bool,
    pub removed: bool,
}

impl Bullet {
    pub fn new_color(id: &'static str, pos: Vec2, vel: Vec2, radius: f32, color: Color) -> Self {
        let base = GameObject::new_with_color(id, pos, vec2(radius, radius), Shape::Circle, color);
        Self {
            base,
            vel,
            active: true,
            removed: false,
        }
    }

    pub async fn new_texture(id: &'static str, pos: Vec2, vel: Vec2, size: Vec2, texture_path: &str) -> Self {
        let texture = load_texture(texture_path).await.unwrap();
        let base = GameObject::new_with_texture(id, pos, size, texture);
        Self {
            base,
            vel,
            active: true,
            removed: false,
        }
    }

    pub fn update(&mut self, dt: f32) {
        if self.active && !self.removed {
            self.base.position += self.vel * dt;
        }
    }

    pub fn draw(&self) {
        if self.active && !self.removed {
            self.base.draw();
        }
    }

    pub fn mark_removed(&mut self) {
        self.removed = true;
        self.active = false;
    }

    pub fn is_offscreen(&self, screen_width: f32, screen_height: f32) -> bool {
        let pos = self.base.position;
        pos.x < 0.0 || pos.x > screen_width || pos.y < 0.0 || pos.y > screen_height
    }

    pub fn as_object(&self) -> &GameObject {
        &self.base
    }
}
