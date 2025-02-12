use macroquad::prelude::*;
use crate::assets::SHOT_IMAGE;
use crate::objects::bullet::Bullet;

use super::objects::GameObject;

const SHOT_SPEED: f32 = 7.0;

pub struct PlayerBullet {
    pub bullet: Bullet,
}

impl PlayerBullet {
    pub async fn new() -> Self {
        let bullet = Bullet::new_texture("player_shot", vec2(1000.0, 1000.0), vec2(0.0, 0.0), vec2(5.0, 5.0), SHOT_IMAGE).await;
        Self { bullet }
    }
    
    pub fn update(&mut self, player_pos: Vec2, mouse_pos: Vec2, _boss_pos: Vec2) {
        if !self.bullet.active && is_mouse_button_pressed(MouseButton::Left) {
            self.bullet.base.position = player_pos;
            self.bullet.vel = (mouse_pos - player_pos).normalize_or_zero() * SHOT_SPEED;
            self.bullet.active = true;
        }
        if self.bullet.active {
            self.bullet.base.position += self.bullet.vel;
            if self.bullet.base.position.x < -10.0 ||
               self.bullet.base.position.x > 630.0 ||
               self.bullet.base.position.y < 10.0 ||
               self.bullet.base.position.y > 470.0 {
                self.bullet.active = false;
                self.bullet.base.position = vec2(1000.0, 1000.0);
            }
        }
    }
    
    pub fn mark_removed(&mut self) {
        self.bullet.mark_removed();
    }
    
    pub fn as_object(&self) -> &GameObject {
        &self.bullet.base
    }
    
    pub fn draw(&self) {
        if self.bullet.active {
            self.bullet.draw();
        }
    }
}
