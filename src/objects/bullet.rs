// src/objects/bullet.rs
use macroquad::prelude::*;

use super::objects::{GameObject, Shape};

pub struct Bullet {
    pub base: GameObject,
    pub vel: Vec2
}

impl Bullet {
    pub fn new_color(id: &'static str, pos: Vec2, vel: Vec2, radius: f32, color: Color) -> Self {
        let base = GameObject::new_with_color(id, pos, vec2(radius, radius), Shape::Circle, color);
        Self {
            base,
            vel
        }
    }

    pub async fn new_texture(id: &'static str, pos: Vec2, vel: Vec2, size: Vec2, texture: Texture2D) -> Self {
        let base = GameObject::new_with_texture(id, pos, size, texture);
        Self {
            base,
            vel
        }
    }

    pub fn update(&mut self, dt: f32) {
        if self.base.active && !self.base.removed {
            self.base.position += self.vel * dt;
        }
    }

    pub fn move_to(&mut self, pos: Vec2) {
        self.base.position = pos;
    }

    pub fn draw(&self) {
        self.base.draw();
    }

    pub fn is_offscreen(&self, screen_width: f32, screen_height: f32) -> bool {
        let pos = self.base.position;
        pos.x < 0.0 || pos.x > screen_width || pos.y < 0.0 || pos.y > screen_height
    }

    pub fn as_object(&self) -> &GameObject {
        &self.base
    }
}
