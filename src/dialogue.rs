// src/dialogue.rs
use macroquad::prelude::*;
use crate::assets::*;

// TO DO: Modularise dialogues.

pub async fn show_menu_dialogue() {
    // TO DO: Play menu music
    let texture = load_texture(MENU_IMAGE_START).await.unwrap();
    loop {
        clear_background(BLACK);
        draw_texture(&texture, 0.0, 0.0, WHITE);
        draw_text("ENTER", 270.0, 322.0, 30.0, Color::new(0.0, 0.0, 0.0, 0.45));
        next_frame().await;
        if is_key_pressed(KeyCode::Enter) {
            break;
        }
    }
    // The texture is dropped when it goes out of scope.
}

pub async fn show_game_over_dialogue() {
    // TO DO: Play menu music
    let texture = load_texture(GAMEOVER_IMAGE).await.unwrap();
    loop {
        clear_background(BLACK);
        draw_texture(&texture, 0.0, 0.0, WHITE);
        next_frame().await;
        if is_key_pressed(KeyCode::Enter) {
            break;
        }
    }
}

pub async fn show_intro_dialogue() {
    let intro_images = [
        DIALOGUE_INTRO_1,
        DIALOGUE_INTRO_2,
        DIALOGUE_INTRO_3,
        DIALOGUE_INTRO_4,
        DIALOGUE_INTRO_5,
        DIALOGUE_INTRO_6,
        DIALOGUE_INTRO_7,
        DIALOGUE_INTRO_8,
        DIALOGUE_INTRO_9,
        DIALOGUE_INTRO_10,
        DIALOGUE_INTRO_11,
        DIALOGUE_INTRO_12,
    ];
    // TO DO: Play menu music
    for image_path in intro_images.iter() {
        let texture = load_texture(image_path).await.unwrap();
        loop {
            clear_background(BLACK);
            draw_texture(&texture, 0.0, 0.0, WHITE);
            next_frame().await;
            if is_key_pressed(KeyCode::Enter) {
                break;
            }
        }
        // Texture is dropped here before loading the next image.
    }
}

pub async fn show_tree_dialogue1() {
    let tree_images = [
        DIALOGUE_ARBOL1_0,
        DIALOGUE_ARBOL1_1,
        DIALOGUE_ARBOL1_2,
        DIALOGUE_ARBOL1_3,
        DIALOGUE_ARBOL1_4,
        DIALOGUE_ARBOL1_5,
        DIALOGUE_ARBOL1_6,
    ];
    // TO DO: Play menu music
    for image_path in tree_images.iter() {
        let texture = load_texture(image_path).await.unwrap();
        loop {
            clear_background(BLACK);
            draw_texture(&texture, 0.0, 0.0, WHITE);
            next_frame().await;
            if is_key_pressed(KeyCode::Enter) {
                break;
            }
        }
    }
    // Show “Press W” screen and wait for W key
    let press_w = load_texture(IMAGE_PRESSW).await.unwrap();
    loop {
        clear_background(BLACK);
        draw_texture(&press_w, 0.0, 0.0, WHITE);
        next_frame().await;
        if is_key_pressed(KeyCode::W) {
            break;
        }
    }
}

pub async fn show_snake_dialogue1() {
    let snake_images = [
        DIALOGUE_SUGEA1_0,
        DIALOGUE_SUGEA1_1,
        DIALOGUE_SUGEA1_2,
        DIALOGUE_SUGEA1_3,
        DIALOGUE_SUGEA1_4,
        DIALOGUE_SUGEA1_5,
    ];
    // TO DO: Play menu music
    for image_path in snake_images.iter() {
        let texture = load_texture(image_path).await.unwrap();
        loop {
            clear_background(BLACK);
            draw_texture(&texture, 0.0, 0.0, WHITE);
            next_frame().await;
            if is_key_pressed(KeyCode::Enter) {
                break;
            }
        }
    }
    let press_w = load_texture(IMAGE_PRESSW).await.unwrap();
    loop {
        clear_background(BLACK);
        draw_texture(&press_w, 0.0, 0.0, WHITE);
        next_frame().await;
        if is_key_pressed(KeyCode::W) {
            break;
        }
    }
}

pub async fn show_octopus_dialogue1() {
    let octopus_images = [
        DIALOGUE_PULPO1_0,
        DIALOGUE_PULPO1_1,
        DIALOGUE_PULPO1_2,
        DIALOGUE_PULPO1_3,
        DIALOGUE_PULPO1_4,
        DIALOGUE_PULPO1_5,
        DIALOGUE_PULPO1_6,
        DIALOGUE_PULPO1_7,
        DIALOGUE_PULPO1_8,
    ];
    // TO DO: Play menu music
    for image_path in octopus_images.iter() {
        let texture = load_texture(image_path).await.unwrap();
        loop {
            clear_background(BLACK);
            draw_texture(&texture, 0.0, 0.0, WHITE);
            next_frame().await;
            if is_key_pressed(KeyCode::Enter) {
                break;
            }
        }
    }
    let press_w = load_texture(IMAGE_PRESSW).await.unwrap();
    loop {
        clear_background(BLACK);
        draw_texture(&press_w, 0.0, 0.0, WHITE);
        next_frame().await;
        if is_key_pressed(KeyCode::W) {
            break;
        }
    }
}

pub async fn show_titan_dialogue1() {
    let titan_images = [
        DIALOGUE_TITAN1_0,
        DIALOGUE_TITAN1_1,
        DIALOGUE_TITAN1_2,
        DIALOGUE_TITAN1_3,
        DIALOGUE_TITAN1_4,
        DIALOGUE_TITAN1_5,
        DIALOGUE_TITAN1_6,
        DIALOGUE_TITAN1_7,
        DIALOGUE_TITAN1_8,
    ];
    // TO DO: Play menu music
    for image_path in titan_images.iter() {
        let texture = load_texture(image_path).await.unwrap();
        loop {
            clear_background(BLACK);
            draw_texture(&texture, 0.0, 0.0, WHITE);
            next_frame().await;
            if is_key_pressed(KeyCode::Enter) {
                break;
            }
        }
    }
    let press_w = load_texture(IMAGE_PRESSW).await.unwrap();
    loop {
        clear_background(BLACK);
        draw_texture(&press_w, 0.0, 0.0, WHITE);
        next_frame().await;
        if is_key_pressed(KeyCode::W) {
            break;
        }
    }
}

pub async fn show_tree_dialogue2() {
    let tree2_images = [
        DIALOGUE_ARBOL2_0,
        DIALOGUE_ARBOL2_1,
        DIALOGUE_ARBOL2_2,
        DIALOGUE_ARBOL2_3,
    ];
    // TO DO: Play menu music
    for image_path in tree2_images.iter() {
        let texture = load_texture(image_path).await.unwrap();
        loop {
            clear_background(BLACK);
            draw_texture(&texture, 0.0, 0.0, WHITE);
            next_frame().await;
            if is_key_pressed(KeyCode::Enter) {
                break;
            }
        }
    }
    let press_w = load_texture(IMAGE_PRESSW).await.unwrap();
    loop {
        clear_background(BLACK);
        draw_texture(&press_w, 0.0, 0.0, WHITE);
        next_frame().await;
        if is_key_pressed(KeyCode::W) {
            break;
        }
    }
}

pub async fn show_snake_dialogue2() {
    let snake2_images = [
        DIALOGUE_SUGEA2_0,
        DIALOGUE_SUGEA2_1,
        DIALOGUE_SUGEA2_2,
        DIALOGUE_SUGEA2_3,
        DIALOGUE_SUGEA2_4,
    ];
    // TO DO: Play menu music
    for image_path in snake2_images.iter() {
        let texture = load_texture(image_path).await.unwrap();
        loop {
            clear_background(BLACK);
            draw_texture(&texture, 0.0, 0.0, WHITE);
            next_frame().await;
            if is_key_pressed(KeyCode::Enter) {
                break;
            }
        }
    }
    let press_w = load_texture(IMAGE_PRESSW).await.unwrap();
    loop {
        clear_background(BLACK);
        draw_texture(&press_w, 0.0, 0.0, WHITE);
        next_frame().await;
        if is_key_pressed(KeyCode::W) {
            break;
        }
    }
}

pub async fn show_octopus_dialogue2b() {
    let octopus2a_images = [
        DIALOGUE_PULPO2_0,
        DIALOGUE_PULPO2_1,
        DIALOGUE_PULPO2_2,
        DIALOGUE_PULPO2_3,
        DIALOGUE_PULPO2_4,
        DIALOGUE_PULPO2_5,
    ];
    // TO DO: Play menu music
    for image_path in octopus2a_images.iter() {
        let texture = load_texture(image_path).await.unwrap();
        loop {
            clear_background(BLACK);
            draw_texture(&texture, 0.0, 0.0, WHITE);
            next_frame().await;
            if is_key_pressed(KeyCode::Enter) {
                break;
            }
        }
    }
    let press_w = load_texture(IMAGE_PRESSW).await.unwrap();
    loop {
        clear_background(BLACK);
        draw_texture(&press_w, 0.0, 0.0, WHITE);
        next_frame().await;
        if is_key_pressed(KeyCode::W) {
            break;
        }
    }
}

pub async fn show_octopus_dialogue2a() {
    let octopus2b_images = [
        DIALOGUE_PULPO2_6,
        DIALOGUE_PULPO2_7,
    ];
    // TO DO: Play menu music
    for image_path in octopus2b_images.iter() {
        let texture = load_texture(image_path).await.unwrap();
        loop {
            clear_background(BLACK);
            draw_texture(&texture, 0.0, 0.0, WHITE);
            next_frame().await;
            if is_key_pressed(KeyCode::Enter) {
                break;
            }
        }
    }
}

pub async fn show_titan_dialogue2a() {
    let titan2a_images = [
        DIALOGUE_TITAN2_0,
        DIALOGUE_TITAN2_1,
        DIALOGUE_TITAN2_2,
        DIALOGUE_TITAN2_3,
        DIALOGUE_TITAN2_4,
        DIALOGUE_TITAN2_5,
        DIALOGUE_TITAN2_6,
        DIALOGUE_TITAN2_7,
        DIALOGUE_TITAN2_8,
        DIALOGUE_TITAN2_9,
    ];
    // TO DO: Play menu music
    for image_path in titan2a_images.iter() {
        let texture = load_texture(image_path).await.unwrap();
        loop {
            clear_background(BLACK);
            draw_texture(&texture, 0.0, 0.0, WHITE);
            next_frame().await;
            if is_key_pressed(KeyCode::Enter) {
                break;
            }
        }
    }
    let press_w = load_texture(IMAGE_PRESSW).await.unwrap();
    loop {
        clear_background(BLACK);
        draw_texture(&press_w, 0.0, 0.0, WHITE);
        next_frame().await;
        if is_key_pressed(KeyCode::W) {
            break;
        }
    }
}

pub async fn show_titan_dialogue2b() {
    let titan2b_images = [
        DIALOGUE_TITAN2_10,
        DIALOGUE_TITAN2_11,
        DIALOGUE_TITAN2_12,
        DIALOGUE_TITAN2_13,
        DIALOGUE_TITAN2_14,
        DIALOGUE_TITAN2_15,
        DIALOGUE_TITAN2_16,
    ];
    // TO DO: Play menu music
    for image_path in titan2b_images.iter() {
        let texture = load_texture(image_path).await.unwrap();
        loop {
            clear_background(BLACK);
            draw_texture(&texture, 0.0, 0.0, WHITE);
            next_frame().await;
            if is_key_pressed(KeyCode::Enter) {
                break;
            }
        }
    }
}
