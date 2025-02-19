use std::i32::MAX;

// src/bosses/boss3.rs
use macroquad::prelude::*;
use crate::{
    assets::*,
    objects::{
        objects::{CollisionType, GameObject},
        player::Player,
        player_bullet::PlayerBullet,
    },
    types::GamePhase,
};
use super::boss::{check_boss_game_state, Boss};

const PLAYER_LIVES: i32 = 6;
const ARM_RADIUS: f32 = 400.0;
const SEGMENT_DIAMETER: f32 = 26.0;

pub struct BossConfig {
    pub life: i32,
    pub speed: f32,
    pub mines: i32
}

fn move_arm(arm_center: Vec2, arm_angle: f32, arm_radius: f32, segments: &mut Vec<GameObject>) {
    for i in 0..segments.len() {
        let segment_radius = arm_radius - (i as f32 * SEGMENT_DIAMETER);
        let segment_pos = vec2(
            segment_radius * arm_angle.cos() + arm_center.x,
            segment_radius * arm_angle.sin() + arm_center.y,
        );
        segments[i].position = segment_pos;
    }
}

fn check_collision_arm_player(player: &mut Player, segments: &mut Vec<GameObject>) -> bool {
    let mut result = false;
    for segment in segments.iter() {
        if segment.collision_type(&player.as_object()) != CollisionType::None {
            result = true;
        }
    }
    result
}

fn draw_arm(arm_center: Vec2, arm_angle: f32, arm_radius: f32, segments: &mut Vec<GameObject>) {
    move_arm(arm_center, arm_angle, arm_radius, segments);
    for segment in segments.iter() {
        segment.draw();
    }
}

pub async fn run_boss_battle(config: BossConfig) -> GamePhase {
    // Load assets.
    let map_texture = load_texture(MAP_BOSS3).await.unwrap();
    let heart_texture = load_texture(PLAYER_HEART_IMAGE).await.unwrap();

    // Create the boss centered on screen.
    let mut boss = Boss::new( vec2(SCREEN_WIDTH / 2.0 - 48.0, SCREEN_HEIGHT / 2.0 - 48.0), config.life, BOSS3_IMAGE).await;
    // Create the player's bullet and player.
    let mut player_bullet = PlayerBullet::new().await;
    let mut player = Player::new(vec2(32.0, 230.0), PLAYER_LIVES).await;

    // arm_center is where the boss’s rotating arm is anchored.
    let arm_center = vec2(SCREEN_WIDTH / 2.0 - 17.0, SCREEN_HEIGHT / 2.0 - 18.0);
    let mut arm_angle: f32 = 0.0;
    let arm_radius = ARM_RADIUS;
    let mut direction: i32 = 1; // 1 for clockwise, -1 for counterclockwise

    let mut invulnerability_timer = 0.0;
    // Compute number of segments for the rotating arm.
    let num_segments = (arm_radius / SEGMENT_DIAMETER) as i32;
    let mut segments : Vec<GameObject> = Vec::new();
    for _ in 0..num_segments {
        segments .push(GameObject::new_with_texture("arm_segment", vec2(0.0, 0.0), vec2(10.0, 10.0), load_texture(BOSS3_WEAPON).await.unwrap()));
    }

    let mut mines : Vec<GameObject> = Vec::new();

    for i in 0..config.mines {
        let random_x = rand::gen_range(i * (SCREEN_WIDTH as i32 / config.mines), i * (SCREEN_WIDTH as i32 / config.mines) + (SCREEN_WIDTH as i32 / config.mines)) as f32;
        let random_y = rand::gen_range(0, SCREEN_HEIGHT as i32) as f32;
        let mine = GameObject::new_with_texture("mine", vec2(random_x, random_y), vec2(23.0, 22.0), load_texture(BOSS3_B_SHOT).await.unwrap());
        mines.push(mine);
    }

    loop {
        let dt = get_frame_time();

        // --- Update phase ---
        player.update_movement();
        player.update_sprite().await;
        let (mx, my) = mouse_position();
        let mouse_vec = vec2(mx, my);
        player_bullet.update(player.base.position, mouse_vec);

        // Update the arm rotation.
        arm_angle += direction as f32 * config.speed;
        // Randomly change rotation direction.
        if rand::gen_range(0, 500) == 300 {
            direction *= -1;
        }

        move_arm(arm_center, arm_angle, arm_radius, &mut segments );

        // --- Collision detection ---
        // Check collision: if the arm hits the player, reduce player life.
        let arm_hit = check_collision_arm_player(&mut player, &mut segments );
        if arm_hit && invulnerability_timer >= 0.1 {
            player.lives -= 1;
            invulnerability_timer = 0.0;
        }

        let player_hit = player.as_object().collision_type(&boss.as_object()) != CollisionType::None;
        if player_hit && invulnerability_timer >= 0.1 {
            player.lives -= 1;
            invulnerability_timer = 0.0;
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

        player_bullet.process_collision(&mut boss).await;

        // --- Draw phase ---
        clear_background(WHITE);
        draw_texture(&map_texture, 0.0, 0.0, WHITE);
        boss.draw();
        player.draw();
        player_bullet.draw();
        draw_arm(arm_center, arm_angle, arm_radius, &mut segments );
        
        if config.mines > 0 {
            for mine in mines.iter() {
                mine.draw();
                println!("{:?}", mine.collision_type(player.as_object()));
            }
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

pub async fn boss3() -> GamePhase {
    let config = BossConfig {
        life: 20,
        speed: 0.0115,
        mines: 0
    };
    run_boss_battle(config).await
}
