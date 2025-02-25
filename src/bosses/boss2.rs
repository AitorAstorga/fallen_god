// src/bosses/boss2.rs
use macroquad::prelude::*;
use crate::{
    assets::*,
    objects::{
        objects::{Appearance, CollisionType},
        player::Player,
        player_bullet::PlayerBullet,
    },
    types::GamePhase,
};
use super::boss::{check_boss_game_state, Boss};

struct BossFollowState {
    has_initial_offset: bool,
    offset: Vec2,
    wall_collision_count: i32,
}

impl BossFollowState {
    fn new() -> Self {
        Self {
            has_initial_offset: false,
            offset: Vec2::ZERO,
            wall_collision_count: 0,
        }
    }
}

fn follow_player(
    boss_pos: &mut Vec2,
    player_pos: Vec2,
    state: &mut BossFollowState,
    speed: f32,
    collision_limit: i32,
) {
    if state.wall_collision_count < collision_limit {
        if !state.has_initial_offset {
            state.offset = player_pos - *boss_pos;
            state.has_initial_offset = true;
        }
        let angle = if state.offset.y != 0.0 && state.offset.x != 0.0 {
            (state.offset.x / state.offset.y).atan()
        } else {
            0.0
        };

        if (state.offset.x < 0.0 && state.offset.y < 0.0)
            || (state.offset.x > 0.0 && state.offset.y < 0.0)
        {
            boss_pos.x -= speed * angle.sin();
            boss_pos.y -= speed * angle.cos();
        } else if (state.offset.x > 0.0 && state.offset.y > 0.0)
            || (state.offset.x < 0.0 && state.offset.y > 0.0)
        {
            boss_pos.x += speed * angle.sin();
            boss_pos.y += speed * angle.cos();
        }

        // Check for boundary collisions ("walls")
        if boss_pos.x < -15.0 || boss_pos.x > 560.0 || boss_pos.y < -12.0 || boss_pos.y > 390.0 {
            state.has_initial_offset = false;
            state.wall_collision_count += 1;
        }
    }
}

async fn update_texture(state: &BossFollowState, boss_shot: Texture2D, boss_normal: Texture2D, collision_limit: i32) -> Texture2D {
    if state.wall_collision_count >= collision_limit {
        boss_normal
    } else {
        boss_shot
    }
}

pub struct BossConfig { 
    // Common boss properties.
    pub collision_limit: i32,

    // Boss A configuration.
    pub boss_a_life: i32,
    pub boss_a_initial_position: Vec2,
    pub boss_a_shot_texture: Texture2D,
    pub boss_a_normal_texture: Texture2D,
    pub boss_a_speed: f32,

    // Boss B configuration (optional).
    pub boss_b_life: Option<i32>,
    pub boss_b_initial_position: Option<Vec2>,
    pub boss_b_shot_texture: Option<Texture2D>,
    pub boss_b_normal_texture: Option<Texture2D>,
    pub boss_b_speed: Option<f32>,
}

pub async fn run_boss_battle(config: BossConfig) -> GamePhase {
    // Load common textures.
    let map_texture = load_texture(MAP_BOSS2).await.unwrap();
    let player_bullet_texture = load_texture(PLAYER_SHOT).await.unwrap();
    let heart_texture = load_texture(PLAYER_HEART_IMAGE).await.unwrap();

    // Create Boss A.
    let mut boss_a = Boss::new(
        config.boss_a_initial_position,
        config.boss_a_life,
        config.boss_a_shot_texture.clone(),
    )
    .await;
    let boss_a_speed = config.boss_a_speed;
    let boss_a_shot = config.boss_a_shot_texture;
    let boss_a_normal = config.boss_a_normal_texture;
    let collision_limit = config.collision_limit;
    let mut state_a = BossFollowState::new();

    // Create Boss B (optional).
    let (mut boss_b, mut state_b) = if let (
        Some(b_life),
        Some(b_initial_pos),
        Some(b_shot),
    ) = (
        config.boss_b_life,
        config.boss_b_initial_position,
        config.boss_b_shot_texture.clone(),
    ) {
        let boss_b = Boss::new(b_initial_pos, b_life, b_shot).await;
        (Some(boss_b), Some(BossFollowState::new()))
    } else {
        (None, None)
    };

    // Create player and player's bullet.
    let mut player = Player::new(vec2(32.0, 230.0), 6).await;
    let mut player_bullet = PlayerBullet::new(player_bullet_texture.clone()).await;

    // Timers for invulnerability and state resets.
    let mut invulnerability_timer = 0.0;
    let mut time_counter = 0;

    loop {
        let dt = get_frame_time();

        // --- Update phase ---
        player.update_movement();
        player.update_sprite();
        let (mx, my) = mouse_position();
        let mouse_vec = vec2(mx, my);

        player_bullet.update(player.base.position, mouse_vec);

        // Update Boss A movement if still alive.
        if boss_a.life > 0 {
            follow_player(&mut boss_a.base.position, player.base.position, &mut state_a, boss_a_speed, collision_limit);
        }
        // Update Boss B movement if it exists and is alive.
        if let (Some(ref mut boss), Some(ref mut state)) = (boss_b.as_mut(), state_b.as_mut()) {
            if boss.life > 0 {
                let b_speed = config.boss_b_speed.unwrap();
                follow_player(&mut boss.base.position, player.base.position, state, b_speed, collision_limit);
            }
        }

        // --- Collision detection ---
        // Process collision for Boss A if it’s still alive.
        if boss_a.life > 0 && player_bullet.as_object().collision_type(&boss_a.as_object()) != CollisionType::None {
            player_bullet.mark_removed();
            player_bullet = PlayerBullet::new(player_bullet_texture.clone()).await;
            if state_a.wall_collision_count == collision_limit {
                boss_a.life -= 1;
                if boss_a.life < 0 {
                    boss_a.life = 0;
                }
            }
        }

        // Process collision for Boss B if it exists and is alive.
        if let (Some(ref mut boss), Some(state)) = (boss_b.as_mut(), state_b.as_ref()) {
            if boss.life > 0 && player_bullet.as_object().collision_type(&boss.as_object()) != CollisionType::None {
                player_bullet.mark_removed();
                player_bullet = PlayerBullet::new(player_bullet_texture.clone()).await;
                if state.wall_collision_count == collision_limit {
                    boss.life -= 1;
                    if boss.life < 0 {
                        boss.life = 0;
                    }
                }
            }
        }

        // If the player collides with any live boss, reduce player lives (with brief invulnerability).
        let collision_a = boss_a.life > 0 && player.as_object().collision_type(&boss_a.as_object()) != CollisionType::None;
        let collision_b = boss_b
            .as_ref()
            .map(|b| b.life > 0 && player.as_object().collision_type(&b.as_object()) != CollisionType::None)
            .unwrap_or(false);
        if (collision_a || collision_b) && invulnerability_timer >= 0.2 {
            player.lives -= 2;
            invulnerability_timer = 0.0;
        }
        invulnerability_timer += dt;

        // Reset boss movement state after enough wall collisions.
        if state_a.wall_collision_count == collision_limit || state_b.as_ref().map(|s| s.wall_collision_count == collision_limit).unwrap_or(false) {
            time_counter += 1;
        }
        if time_counter >= 300 {
            time_counter = 0;
            state_a.wall_collision_count = 0;
            if let Some(ref mut state) = state_b {
                state.wall_collision_count = 0;
            }
        }

        // --- Update textures based on collision state ---
        let new_tex_a = update_texture(&state_a, boss_a_shot.clone(), boss_a_normal.clone(), collision_limit).await;
        boss_a.base.appearance = Appearance::Texture(new_tex_a);
        if let (Some(ref mut boss), Some(state)) = (boss_b.as_mut(), state_b.as_ref()) {
            let boss_b_shot = config.boss_b_shot_texture.clone().unwrap();
            let boss_b_normal = config.boss_b_normal_texture.clone().unwrap();
            let new_tex_b = update_texture(state, boss_b_shot.clone(), boss_b_normal, collision_limit).await;
            boss.base.appearance = Appearance::Texture(new_tex_b);
        }

        // --- Draw phase ---
        clear_background(WHITE);
        draw_texture(&map_texture, 0.0, 0.0, WHITE);
        if boss_a.life > 0 {
            boss_a.draw();
        }
        if let Some(ref boss) = boss_b {
            if boss.life > 0 {
                boss.draw();
            }
        }
        player.draw();
        player_bullet.draw();

        // Draw player hearts.
        for i in 0..player.lives {
            draw_texture(&heart_texture, i as f32 * 40.0, 0.0, WHITE);
        }

        // --- Check overall game state ---
        let boss_a_alive = boss_a.life > 0;
        let boss_b_alive = boss_b.as_ref().map(|b| b.life > 0).unwrap_or(false);
        let state_phase = if !boss_a_alive && !boss_b_alive {
            // Both bosses are dead: win the battle.
            check_boss_game_state(player.lives, 0)
        } else {
            // Battle continues.
            check_boss_game_state(player.lives, boss_a.life.max(0) + boss_b.as_ref().map(|b| b.life.max(0)).unwrap_or(0))
        };

        next_frame().await;
        if state_phase != GamePhase::Boss {
            return state_phase;
        }
    }
}

pub async fn boss2() -> GamePhase {
    let config = BossConfig {
        collision_limit: 5,
        boss_a_life: 20,
        boss_a_initial_position: vec2(500.0, 225.0),
        boss_a_shot_texture: load_texture(BOSS2_SHOT).await.unwrap(),
        boss_a_normal_texture: load_texture(BOSS2_IMAGE).await.unwrap(),
        boss_a_speed: 4.0,
        // For a single-boss battle, leave Boss B as None.
        boss_b_life: None,
        boss_b_initial_position: None,
        boss_b_shot_texture: None,
        boss_b_normal_texture: None,
        boss_b_speed: None,
    };
    run_boss_battle(config).await
}
