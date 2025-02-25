// src/player.rs
use macroquad::prelude::*;
use crate::{assets::*, objects::objects::{Appearance, GameObject}};

const PLAYER_BASE_SPEED: f32 = 7.0;
const DASH_MULTIPLIER: f32 = 2.5;
const DASH_DURATION: f32 = 0.2;
const DASH_COOLDOWN: f32 = 1.0;

pub struct PlayerTextures {
    pub front1: Texture2D,
    pub front2: Texture2D,
    pub back1: Texture2D,
    pub back2: Texture2D,
    pub left1: Texture2D,
    pub left2: Texture2D,
    pub right1: Texture2D,
    pub right2: Texture2D,
}

pub struct Player {
    pub base: GameObject,
    pub lives: i32,
    pub dash_timer: f32,
    pub dash_cooldown_timer: f32,
    pub dashing: bool,
    sprite: bool,
    textures: PlayerTextures,
}

impl Player {
    pub async fn new(initial_pos: Vec2, lives: i32) -> Self {
        // Preload all textures asynchronously just once
        let front1 = load_texture(PLAYER_IMAGE_FRONT1).await.unwrap();
        let front2 = load_texture(PLAYER_IMAGE_FRONT2).await.unwrap();
        let back1 = load_texture(PLAYER_IMAGE_BACK1).await.unwrap();
        let back2 = load_texture(PLAYER_IMAGE_BACK2).await.unwrap();
        let left1 = load_texture(PLAYER_IMAGE_LEFT1).await.unwrap();
        let left2 = load_texture(PLAYER_IMAGE_LEFT2).await.unwrap();
        let right1 = load_texture(PLAYER_IMAGE_RIGHT1).await.unwrap();
        let right2 = load_texture(PLAYER_IMAGE_RIGHT2).await.unwrap();

        let textures = PlayerTextures {
            front1, front2,
            back1, back2,
            left1, left2,
            right1, right2
        };

        let base = GameObject::new_with_texture("player", initial_pos, vec2(PLAYER_SIZE_X, PLAYER_SIZE_Y), textures.front1.clone());

        Self {
            base,
            lives,
            dash_timer: 0.0,
            dash_cooldown_timer: 0.0,
            dashing: false,
            sprite: false,
            textures
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

        if is_key_down(KeyCode::D) && self.base.position.x < SCREEN_WIDTH - PLAYER_SIZE_X {
            self.base.position.x += speed * dt * 60.0;
        }
        if is_key_down(KeyCode::S) && self.base.position.y < SCREEN_HEIGHT - PLAYER_SIZE_Y {
            self.base.position.y += speed * dt * 60.0;
        }
        if is_key_down(KeyCode::A) && self.base.position.x > 0.0 {
            self.base.position.x -= speed * dt * 60.0;
        }
        if is_key_down(KeyCode::W) && self.base.position.y > 0.0 {
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
    
    pub fn update_sprite(&mut self) {
        if is_key_pressed(KeyCode::A) {
            if self.sprite {
                self.base.appearance = Appearance::Texture(self.textures.left1.clone());
            } else {
                self.base.appearance = Appearance::Texture(self.textures.left2.clone());
            }
            self.sprite = !self.sprite;
        }
        if is_key_pressed(KeyCode::S) {
            if self.sprite {
                self.base.appearance = Appearance::Texture(self.textures.front1.clone());
            } else {
                self.base.appearance = Appearance::Texture(self.textures.front2.clone());
            }
            self.sprite = !self.sprite;
        }
        if is_key_pressed(KeyCode::D) {
            if self.sprite {
                self.base.appearance = Appearance::Texture(self.textures.right1.clone());
            } else {
                self.base.appearance = Appearance::Texture(self.textures.right2.clone());
            }
            self.sprite = !self.sprite;
        }
        if is_key_pressed(KeyCode::W) {
            if self.sprite {
                self.base.appearance = Appearance::Texture(self.textures.back1.clone());
            } else {
                self.base.appearance = Appearance::Texture(self.textures.back2.clone());
            }
            self.sprite = !self.sprite;
        }
    }
    
    pub fn draw(&self) {
        self.base.draw();
    }
    
    pub fn as_object(&self) -> &GameObject {
        &self.base
    }
}
