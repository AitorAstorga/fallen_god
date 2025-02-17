// src/bosses/boss2.rs
use macroquad::prelude::*;
use crate::{
    assets::*,
    objects::{
        objects::{CollisionType, Appearance},
        player::Player,
        player_bullet::PlayerBullet,
    },
    types::GamePhase,
};
use super::boss::{check_boss_game_state, Boss};

const BOSS_WALL_COLLISIONS: i32 = 5;
const BOSS_LIFE: i32 = 20;
const PLAYER_LIVES: i32 = 6;

struct Boss2State {
    has_initial_offset: bool,
    offset_x: f32,
    offset_y: f32,
    wall_collision_count: i32,
}

impl Boss2State {
    fn new() -> Self {
        Self {
            has_initial_offset: false,
            offset_x: 0.0,
            offset_y: 0.0,
            wall_collision_count: 0,
        }
    }
}

fn boss2_follow_player(boss_pos: &mut Vec2, player_pos: Vec2, state: &mut Boss2State) {
    if state.wall_collision_count < BOSS_WALL_COLLISIONS {
        if !state.has_initial_offset {
            state.offset_x = player_pos.x - boss_pos.x;
            state.offset_y = player_pos.y - boss_pos.y;
            state.has_initial_offset = true;
        }
        let mut angle = 0.0;
        if state.offset_y != 0.0 && state.offset_x != 0.0 {
            angle = (state.offset_x / state.offset_y).atan();
        }
        if (state.offset_x < 0.0 && state.offset_y < 0.0)
            || (state.offset_x > 0.0 && state.offset_y < 0.0)
        {
            boss_pos.x -= 4.0 * angle.sin();
            boss_pos.y -= 4.0 * angle.cos();
        } else if (state.offset_x > 0.0 && state.offset_y > 0.0)
            || (state.offset_x < 0.0 && state.offset_y > 0.0)
        {
            boss_pos.x += 4.0 * angle.sin();
            boss_pos.y += 4.0 * angle.cos();
        }
        if boss_pos.x < -15.0 || boss_pos.x > 560.0 || boss_pos.y < -12.0 || boss_pos.y > 390.0 {
            state.has_initial_offset = false;
            state.wall_collision_count += 1;
        }
    }
}

/// If the boss has “hit a wall” enough times, update its texture.
async fn update_serpiente_texture(state: &Boss2State) -> Texture2D {
    // If wall collisions reached 5 or more, switch to the "serpiente1" texture.
    if state.wall_collision_count >= BOSS_WALL_COLLISIONS {
        load_texture(BOSS2_IMAGE).await.unwrap()
    } else {
        load_texture(BOSS2_SHOT).await.unwrap()
    }
}

pub async fn boss2() -> GamePhase {
    // Load map and create boss with initial position.
    let map_texture = load_texture(MAP_BOSS2).await.unwrap();
    let mut boss = Boss::new(vec2(500.0, 225.0), BOSS_LIFE, BOSS2_SHOT).await;
    let mut player_bullet = PlayerBullet::new().await;
    let mut player = Player::new(vec2(32.0, 230.0), PLAYER_LIVES).await;
    let heart_texture = load_texture(PLAYER_HEART_IMAGE).await.unwrap();
    let mut state = Boss2State::new();
    // Counters for applying player damage and for texture changes.
    let mut invulnerability_timer = 0.0;
    let mut time_counter = 0;

    loop {
        let dt = get_frame_time();

        // Update player and player's shot.
        player.update_movement();
        player.update_sprite().await;
        let (mx, my) = mouse_position();
        let mouse_vec = vec2(mx, my);
        player_bullet.update(player.base.position, mouse_vec, boss.base.position);

        // Update boss movement by following the player.
        boss2_follow_player(&mut boss.base.position, player.base.position, &mut state);

        // Check collision: if player's shot hits the boss, reduce boss life.
        if player_bullet.as_object().collision_type(&boss.as_object()) != CollisionType::None {
            player_bullet.mark_removed();
            player_bullet = PlayerBullet::new().await;
            if  state.wall_collision_count == BOSS_WALL_COLLISIONS {
                boss.life -= 1;
            }
        }

        // Check collision: if player hits the boss, reduce player lives.
        if player.as_object().collision_type(&boss.as_object()) != CollisionType::None && invulnerability_timer >= 0.1 {
            player.lives -= 2;
            invulnerability_timer = 0.0;
        }
        invulnerability_timer += dt;

        // Wait for 300 frames to resume boss movement.
        if state.wall_collision_count == BOSS_WALL_COLLISIONS {
            time_counter += 1;
        }
        if time_counter >= 300 {
            time_counter = 0;
            state.wall_collision_count = 0;
        }

        if state.wall_collision_count == 0 || state.wall_collision_count == BOSS_WALL_COLLISIONS {
            let new_tex = update_serpiente_texture(&state).await;
            boss.base.appearance = Appearance::Texture(new_tex);
        }

        // Draw scene.
        clear_background(WHITE);
        draw_texture(&map_texture, 0.0, 0.0, WHITE);
        boss.draw();
        player.draw();
        player_bullet.draw();

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
