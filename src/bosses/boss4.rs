// src/bosses/boss4.rs

use macroquad::prelude::*;
use crate::{
    assets::*,
    objects::{
        bullet::Bullet,
        objects::{Appearance, CollisionType, GameObject},
        player::Player,
        player_bullet::PlayerBullet,
    },
    types::GamePhase,
};
use super::boss::{check_boss_game_state, Boss};

const PLAYER_LIVES: i32 = 6;

pub struct BossConfig {
    pub life: i32,
    pub bullet_speed_multiplier: f32,
    pub mines: i32,
    pub boss_texture_left: Texture2D,
    pub boss_texture_right: Texture2D,
    pub map_texture: Texture2D
}

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
    if !proj.base.removed {
        proj.move_to(proj.as_object().position + velocity);
        if proj.as_object().position.x < 0.0
            || proj.as_object().position.x > SCREEN_WIDTH
            || proj.as_object().position.y < 0.0
            || proj.as_object().position.y > SCREEN_HEIGHT
        {
            proj.base.mark_removed();
        }
    }
}

pub async fn run_boss_battle(config: BossConfig) -> GamePhase {
    // Load map texture and create boss.
    let map_texture = config.map_texture;
    let player_bullet_texture = load_texture(PLAYER_SHOT).await.unwrap();
    let boss_bullet_texture = load_texture(BOSS4_SHOT).await.unwrap();

    let mut boss = Boss::new(vec2(500.0, 180.0), config.life, config.boss_texture_left.clone()).await;

    // Create player's shot and the player.
    let mut player_bullet = PlayerBullet::new(player_bullet_texture.clone()).await;
    let mut player = Player::new(vec2(32.0, 230.0), PLAYER_LIVES).await;
    let heart_texture = load_texture(PLAYER_HEART_IMAGE).await.unwrap();

    // Create boss bullets in a formation.
    let spawn_pos = boss.base.position + vec2(50.0, 30.0);
    let mut proj1 = Bullet::new_texture("boss_proj", spawn_pos, vec2(0.0, 0.0), vec2(32.0 , 32.0 ), boss_bullet_texture.clone()).await;
    let mut proj2 = Bullet::new_texture("boss_proj", spawn_pos + vec2(30.0, 30.0), vec2(0.0, 0.0), vec2(32.0 , 32.0 ), boss_bullet_texture.clone()).await;
    let mut proj3 = Bullet::new_texture("boss_proj", spawn_pos + vec2(30.0, -30.0), vec2(0.0, 0.0), vec2(32.0 , 32.0 ), boss_bullet_texture.clone()).await;
    // We'll compute a single common velocity for the formation.
    let mut formation_velocity: Option<Vec2> = None;

    // Time counter for boss movement.
    let mut time_counter: u32 = 0;
    // Timer for player invulnerability.
    let mut invulnerability_timer = 0.0;

    let mut mines : Vec<GameObject> = Vec::new();

    for i in 0..config.mines {
        let random_x = rand::gen_range(i * (SCREEN_WIDTH as i32 / config.mines), i * (SCREEN_WIDTH as i32 / config.mines) + (SCREEN_WIDTH as i32 / config.mines)) as f32;
        let random_y = rand::gen_range(0, SCREEN_HEIGHT as i32) as f32;
        let mine = GameObject::new_with_texture("mine", vec2(random_x, random_y), vec2(23.0, 22.0), load_texture(BOSS3_B_SHOT).await.unwrap());
        mines.push(mine);
    }

    let boss_left_texture = config.boss_texture_left;
    let boss_right_texture = config.boss_texture_right;

    loop {
        let dt = get_frame_time();
        time_counter = (time_counter + 1) % 2700;

        // --- Update phase ---
        // Update player and player's shot.
        player.update_movement();
        player.update_sprite();
        let (mx, my) = mouse_position();
        let mouse_vec = vec2(mx, my);
        player_bullet.update(player.base.position, mouse_vec);

        // Initialise the boss shots velocity.
        if formation_velocity.is_none() {
            let leader_spawn = boss.base.position + vec2(50.0, 30.0);
            formation_velocity = Some((player.base.position - leader_spawn).normalize() * config.bullet_speed_multiplier);
        }
        let velocity = formation_velocity.unwrap();

        // Update each projectile (if not removed) by adding the common velocity.
        update_projectile(&mut proj1, velocity);
        update_projectile(&mut proj2, velocity);
        update_projectile(&mut proj3, velocity);

        move_boss(&mut boss, time_counter);

        // Update boss sprite based on player's position.
        if boss.base.position.x > player.base.position.x {
            boss.base.appearance = Appearance::Texture(boss_right_texture.clone());
        } else {
            boss.base.appearance = Appearance::Texture(boss_left_texture.clone());
        }

        // --- Collision detection ---
        // Check collision: if player's shot hits the boss.
        if player_bullet.as_object().collision_type(&boss.as_object()) != CollisionType::None {
            boss.life -= 1;
            player_bullet.mark_removed();
            player_bullet = PlayerBullet::new(player_bullet_texture.clone()).await;
        }

        // Check collision for each boss projectile with the player.
        for proj in [&mut proj1, &mut proj2, &mut proj3].iter_mut() {
            if !proj.base.removed
                && player.as_object().collision_type(&proj.as_object()) != CollisionType::None
                && invulnerability_timer >= 0.1
            {
                player.lives -= 1;
                invulnerability_timer = 0.0;
                proj.base.mark_removed();
            }
        }
        invulnerability_timer += dt;

        if config.mines > 0 {
            for mine in mines.iter_mut() {
                if mine.collision_type(player.as_object()) != CollisionType::None {
                    player.lives -= 1;
                    mine.mark_removed();
                }
            }
        }

        // When all three projectiles are removed, reinitialise the formation
        if proj1.base.removed && proj2.base.removed && proj3.base.removed {
            let spawn_pos = boss.base.position + vec2(50.0, 30.0);
            proj1 = Bullet::new_texture("boss_proj", spawn_pos, vec2(0.0, 0.0), vec2(32.0 , 32.0 ), boss_bullet_texture.clone()).await;
            proj2 = Bullet::new_texture("boss_proj", spawn_pos + vec2(30.0, 30.0), vec2(0.0, 0.0), vec2(32.0 , 32.0 ), boss_bullet_texture.clone()).await;
            proj3 = Bullet::new_texture("boss_proj", spawn_pos + vec2(30.0, -30.0), vec2(0.0, 0.0), vec2(32.0 , 32.0 ), boss_bullet_texture.clone()).await;
            formation_velocity = None;
        }

        // --- Draw phase ---
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

        if config.mines > 0 {
            for mine in mines.iter() {
                mine.draw();
            }
        }

        let state = check_boss_game_state(player.lives, boss.life);
        next_frame().await;
        if state != GamePhase::Boss {
            return state;
        }
    }
}

pub async fn boss4() -> GamePhase {
    let config = BossConfig {
        life: 20,
        bullet_speed_multiplier: 5.0,
        mines: 0,
        boss_texture_left: load_texture(BOSS4_LEFT_IMAGE).await.unwrap(),
        boss_texture_right: load_texture(BOSS4_RIGHT_IMAGE).await.unwrap(),
        map_texture: load_texture(MAP_BOSS4).await.unwrap()
    };
    run_boss_battle(config).await
}
