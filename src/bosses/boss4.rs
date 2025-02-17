// src/bosses/boss4.rs

use macroquad::prelude::*;
use crate::{
    assets::*,
    objects::{
        bullet::Bullet,
        objects::{Appearance, CollisionType},
        player::Player,
        player_bullet::PlayerBullet,
    },
    types::GamePhase,
};
use super::boss::{check_boss_game_state, Boss};

const BOSS_LIVES: i32 = 20;
const PLAYER_LIVES: i32 = 6;
const BOSS_BULLET_SPEED_MULTIPLIER: f32 = 5.0;

fn move_boss(boss: &mut Boss, time_counter: u32) {
    if time_counter > 400 && time_counter < 600 {
        boss.base.position.y += 1.0;
    }
    if time_counter > 650 && time_counter < 1150 {
        boss.base.position.x -= 1.0;
    }
    if time_counter > 1250 && time_counter < 1600 {
        boss.base.position.y -= 1.0;
    }
    if time_counter > 1800 && time_counter < 2300 {
        boss.base.position.x += 1.0;
    }
    if time_counter > 2500 && time_counter < 2650 {
        boss.base.position.y += 1.0;
    }
}

fn update_projectile(proj: &mut Bullet, velocity: Vec2) {
    if !proj.removed {
        proj.move_to(proj.as_object().position + velocity);
        if proj.as_object().position.x < 0.0
            || proj.as_object().position.x > SCREEN_WIDTH
            || proj.as_object().position.y < 0.0
            || proj.as_object().position.y > SCREEN_HEIGHT
        {
            proj.mark_removed();
        }
    }
}

pub async fn boss4() -> GamePhase {
    // Load map texture and create boss.
    let map_texture = load_texture(MAP_BOSS4).await.unwrap();
    let mut boss = Boss::new(vec2(500.0, 180.0), BOSS_LIVES, BOSS4_LEFT_IMAGE).await;

    // Create player's shot and the player.
    let mut player_bullet = PlayerBullet::new().await;
    let mut player = Player::new(vec2(32.0, 230.0), PLAYER_LIVES).await;
    let heart_texture = load_texture(PLAYER_HEART_IMAGE).await.unwrap();

    // Create boss bullets in a formation.
    let spawn_pos = boss.base.position + vec2(50.0, 30.0);
    let mut proj1 = Bullet::new_texture("boss_proj", spawn_pos, vec2(0.0, 0.0), vec2(32.0 , 32.0 ), BOSS4_SHOT).await;
    let mut proj2 = Bullet::new_texture("boss_proj", spawn_pos + vec2(30.0, 30.0), vec2(0.0, 0.0), vec2(32.0 , 32.0 ), BOSS4_SHOT).await;
    let mut proj3 = Bullet::new_texture("boss_proj", spawn_pos + vec2(30.0, -30.0), vec2(0.0, 0.0), vec2(32.0 , 32.0 ), BOSS4_SHOT).await;
    // We'll compute a single common velocity for the formation.
    let mut formation_velocity: Option<Vec2> = None;

    // Time counter for boss movement.
    let mut time_counter: u32 = 0;
    // Timer for player invulnerability.
    let mut invulnerability_timer = 0.0;

    loop {
        let dt = get_frame_time();
        time_counter = (time_counter + 1) % 2700;

        // Update player and player's shot.
        player.update_movement();
        player.update_sprite().await;
        let (mx, my) = mouse_position();
        let mouse_vec = vec2(mx, my);
        player_bullet.update(player.base.position, mouse_vec, boss.base.position);

        // Initialise the boss shots velocity.
        if formation_velocity.is_none() {
            let leader_spawn = boss.base.position + vec2(50.0, 30.0);
            formation_velocity = Some((player.base.position - leader_spawn).normalize() * BOSS_BULLET_SPEED_MULTIPLIER);
        }
        let velocity = formation_velocity.unwrap();

        // Update each projectile (if not removed) by adding the common velocity.
        update_projectile(&mut proj1, velocity);
        update_projectile(&mut proj2, velocity);
        update_projectile(&mut proj3, velocity);

        move_boss(&mut boss, time_counter);

        // Update boss sprite based on player's position.
        if boss.base.position.x > player.base.position.x {
            boss.base.appearance = Appearance::Texture(load_texture(BOSS4_RIGHT_IMAGE).await.unwrap());
        } else {
            boss.base.appearance = Appearance::Texture(load_texture(BOSS4_LEFT_IMAGE).await.unwrap());
        }

        // Check collision: if player's shot hits the boss.
        if player_bullet.as_object().collision_type(&boss.as_object()) != CollisionType::None {
            boss.life -= 1;
            player_bullet.mark_removed();
            player_bullet = PlayerBullet::new().await;
        }

        // Check collision for each boss projectile with the player.
        for proj in [&mut proj1, &mut proj2, &mut proj3].iter_mut() {
            if !proj.removed
                && player.as_object().collision_type(&proj.as_object()) != CollisionType::None
                && invulnerability_timer >= 0.1
            {
                player.lives -= 1;
                invulnerability_timer = 0.0;
                proj.mark_removed();
            }
        }
        invulnerability_timer += dt;

        // When all three projectiles are removed, reinitialise the formation
        if proj1.removed && proj2.removed && proj3.removed {
            let spawn_pos = boss.base.position + vec2(50.0, 30.0);
            proj1 = Bullet::new_texture("boss_proj", spawn_pos, vec2(0.0, 0.0), vec2(32.0 , 32.0 ), BOSS4_SHOT).await;
            proj2 = Bullet::new_texture("boss_proj", spawn_pos + vec2(30.0, 30.0), vec2(0.0, 0.0), vec2(32.0 , 32.0 ), BOSS4_SHOT).await;
            proj3 = Bullet::new_texture("boss_proj", spawn_pos + vec2(30.0, -30.0), vec2(0.0, 0.0), vec2(32.0 , 32.0 ), BOSS4_SHOT).await;
            formation_velocity = None;
        }

        // Draw the scene.
        clear_background(WHITE);
        draw_texture(&map_texture, 0.0, 0.0, WHITE);
        boss.draw();
        player.draw();
        player_bullet.draw();
        proj1.draw();
        proj2.draw();
        proj3.draw();
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
