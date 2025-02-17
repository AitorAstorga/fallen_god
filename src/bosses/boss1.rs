// src/bosses/boss1.rs
use macroquad::prelude::*;
use crate::{
    assets::*,
    objects::{
        bullet::Bullet,
        objects::CollisionType,
        player::Player,
        player_bullet::PlayerBullet,
    },
    types::GamePhase,
};
use super::boss::{check_boss_game_state, Boss};

const BOSS_SHOOT_INTERVAL: f32 = 1.0;
const BOSS_BULLET_SPEED: f32 = 200.0;
const BOSS_SHOT_PRECISION: f32 = 0.75;
const BOSS_LIFE: i32 = 20;
const PLAYER_LIVES: i32 = 6;

pub async fn boss1() -> GamePhase {
    let map_texture = load_texture(MAP_BOSS1).await.unwrap();
    let mut boss = Boss::new(vec2(275.0, 195.0), BOSS_LIFE, BOSS1_IMAGE).await;
    let mut player_bullet = PlayerBullet::new().await;
    let mut player = Player::new(vec2(32.0, 230.0), PLAYER_LIVES).await;
    let heart_texture = load_texture(PLAYER_HEART_IMAGE).await.unwrap();
    let mut boss_bullets: Vec<Bullet> = Vec::new();
    let mut shoot_timer = 0.0;
    let mut invulnerability_timer = 0.0;
    
    loop {
        let dt = get_frame_time();
        
        // Update player and shot.
        player.update_movement();
        player.update_sprite().await;
        let (mx, my) = mouse_position();
        let mouse_vec = vec2(mx, my);
        player_bullet.update(player.base.position, mouse_vec, boss.base.position);
        
        // Boss shooting logic.
        shoot_timer += dt;
        if shoot_timer >= BOSS_SHOOT_INTERVAL {
            shoot_timer = 0.0;
            let aim_dir = (player.base.position - boss.base.position).normalize_or_zero();
            let random_angle = rand::gen_range(0.0, 2.0 * std::f32::consts::PI);
            let random_dir = vec2(random_angle.cos(), random_angle.sin());
            let final_dir = (((1.0 - BOSS_SHOT_PRECISION) * random_dir) + (BOSS_SHOT_PRECISION * aim_dir)).normalize_or_zero();
            let bullet_vel = final_dir * BOSS_BULLET_SPEED;
            boss_bullets.push(Bullet::new_texture("boss_bullet", boss.base.position, bullet_vel, vec2(25.0, 25.0), BOSS1_SHOT).await);
        }
        
        // Update boss bullets.
        for bullet in boss_bullets.iter_mut() {
            bullet.update(dt);
        }
        boss_bullets.retain(|b| !b.removed && !b.is_offscreen(SCREEN_WIDTH, SCREEN_HEIGHT));
        
        // Check collisions: if a boss bullet collides with the player, mark the bullet for removal.
        for bullet in &mut boss_bullets {
            if player.as_object().collision_type(&bullet.as_object()) != CollisionType::None && invulnerability_timer >= 0.1 {
                player.lives -= 1;
                invulnerability_timer = 0.0;
                bullet.mark_removed();
                // TO DO: Bullet sound?
            }
        }
        invulnerability_timer += dt;
        
        player_bullet.process_collision(&mut boss).await;
        
        // Draw the scene.
        clear_background(WHITE);
        draw_texture(&map_texture, 0.0, 0.0, WHITE);
        boss.draw();
        player.draw();
        player_bullet.draw();
        for bullet in &boss_bullets {
            bullet.draw();
        }
        
        // Draw the lives as hearts.
        for i in 0..player.lives {
            draw_texture(&heart_texture, i as f32 * 40.0, 0.0, WHITE);
        }
        
        let state = check_boss_game_state(player.lives, boss.life);
        next_frame().await;
        if state != GamePhase::Boss {
            return state;
        }
    }
}
