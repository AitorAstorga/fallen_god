use macroquad::prelude::*;
use crate::assets::{PLAYER_SHOT, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::bosses::boss::Boss;
use crate::objects::objects::CollisionType;
use crate::objects::bullet::Bullet;
use super::objects::GameObject;

const SHOT_SPEED: f32 = 7.0;

pub struct PlayerBullet {
    pub bullet: Bullet,
}

impl PlayerBullet {
    pub async fn new() -> Self {
        let bullet = Self::get_initial_bullet().await;
        Self { bullet }
    }

    pub async fn get_initial_bullet() -> Bullet {
        let bullet = Bullet::new_texture("player_shot", vec2(1000.0, 1000.0), vec2(0.0, 0.0), vec2(5.0, 5.0), PLAYER_SHOT).await;
        bullet
    }
    
    pub fn update(&mut self, player_pos: Vec2, mouse_pos: Vec2) {
        if !self.bullet.base.active && is_mouse_button_pressed(MouseButton::Left) {
            self.bullet.base.position = player_pos;
            self.bullet.vel = (mouse_pos - player_pos).normalize_or_zero() * SHOT_SPEED;
            self.bullet.base.active = true;
        }
        if self.bullet.base.active {
            self.bullet.base.position += self.bullet.vel;
            if self.bullet.base.position.x < 0.0 ||
               self.bullet.base.position.x > SCREEN_WIDTH ||
               self.bullet.base.position.y < 0.0 ||
               self.bullet.base.position.y > SCREEN_HEIGHT {
                self.bullet.base.active = false;
                self.bullet.base.position = vec2(1000.0, 1000.0);
            }
        }
    }

    pub async fn process_collision(&mut self, boss: &mut Boss) {
        if self.as_object().collision_type(boss.as_object()) != CollisionType::None {
            boss.life -= 1;
            self.mark_removed();
            self.bullet = PlayerBullet::get_initial_bullet().await;
        }
    }
    
    pub fn mark_removed(&mut self) {
        self.bullet.base.mark_removed();
    }
    
    pub fn as_object(&self) -> &GameObject {
        &self.bullet.base
    }
    
    pub fn draw(&self) {
        if self.bullet.base.active {
            self.bullet.draw();
        }
    }
}
