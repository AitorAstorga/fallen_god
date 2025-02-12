// src/player.rs
use macroquad::prelude::*;
use crate::{assets::*, objects::objects::{Appearance, GameObject}};

const PLAYER_BASE_SPEED: f32 = 7.0;
const DASH_MULTIPLIER: f32 = 2.5;
const DASH_DURATION: f32 = 0.2;
const DASH_COOLDOWN: f32 = 1.0;

pub struct Player {
    pub base: GameObject,
    pub lives: i32,
    pub dash_timer: f32,
    pub dash_cooldown_timer: f32,
    pub dashing: bool,
    sprite: bool,
}

impl Player {
    pub async fn new(initial_pos: Vec2) -> Self {
        let texture = load_texture(PLAYER_IMAGE_FRONT1).await.unwrap();
        let base = GameObject::new_with_texture("player", initial_pos, vec2(34.0, 50.0), texture);
        Self {
            base,
            lives: 6,
            dash_timer: 0.0,
            dash_cooldown_timer: 0.0,
            dashing: false,
            sprite: false,
        }
    }
    
    pub fn update_movement(&mut self) {
        let dt = get_frame_time();
        if is_key_pressed(KeyCode::Space) && self.dash_cooldown_timer <= 0.0 {
            self.dashing = true;
            self.dash_timer = DASH_DURATION;
            self.dash_cooldown_timer = DASH_COOLDOWN;
        }
        let speed = if self.dashing { PLAYER_BASE_SPEED * DASH_MULTIPLIER } else { PLAYER_BASE_SPEED };

        if is_key_down(KeyCode::D) && self.base.position.x < 580.0 {
            self.base.position.x += speed * dt * 60.0;
        }
        if is_key_down(KeyCode::S) && self.base.position.y < 440.0 {
            self.base.position.y += speed * dt * 60.0;
        }
        if is_key_down(KeyCode::A) && self.base.position.x > -5.0 {
            self.base.position.x -= speed * dt * 60.0;
        }
        if is_key_down(KeyCode::W) && self.base.position.y > 20.0 {
            self.base.position.y -= speed * dt * 60.0;
        }
        
        if self.dashing {
            self.dash_timer -= dt;
            if self.dash_timer <= 0.0 { self.dashing = false; }
        }
        if self.dash_cooldown_timer > 0.0 {
            self.dash_cooldown_timer -= dt;
        }
    }
    
    pub async fn update_sprite(&mut self) {
        if is_key_pressed(KeyCode::D) {
            if self.sprite {
                self.base.appearance = Appearance::Texture(load_texture(PLAYER_IMAGE_RIGHT1).await.unwrap());
            } else {
                self.base.appearance = Appearance::Texture(load_texture(PLAYER_IMAGE_RIGHT2).await.unwrap());
            }
            self.sprite = self.sprite ^ true;
        }
        if is_key_pressed(KeyCode::S) {
            if self.sprite {
                self.base.appearance = Appearance::Texture(load_texture(PLAYER_IMAGE_FRONT1).await.unwrap());
            } else {
                self.base.appearance = Appearance::Texture(load_texture(PLAYER_IMAGE_FRONT2).await.unwrap());
            }
            self.sprite = self.sprite ^ true;
        }
        if is_key_pressed(KeyCode::A) {
            if self.sprite {
                self.base.appearance = Appearance::Texture(load_texture(PLAYER_IMAGE_LEFT1).await.unwrap());
            } else {
                self.base.appearance = Appearance::Texture(load_texture(PLAYER_IMAGE_LEFT2).await.unwrap());
            }
            self.sprite = self.sprite ^ true;
        }
        if is_key_pressed(KeyCode::W) {
            if self.sprite {
                self.base.appearance = Appearance::Texture(load_texture(PLAYER_IMAGE_BACK1).await.unwrap());
            } else {
                self.base.appearance = Appearance::Texture(load_texture(PLAYER_IMAGE_BACK2).await.unwrap());
            }
            self.sprite = self.sprite ^ true;
        }
    }
    
    pub fn draw(&self) {
        self.base.draw();
    }
    
    pub fn as_object(&self) -> &GameObject {
        &self.base
    }
}
