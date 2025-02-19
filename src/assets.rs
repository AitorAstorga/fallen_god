// src/assets.rs

pub struct Image {
    pub path: &'static str,
    pub width: f32,
    pub height: f32,
}

// ==========================
// Dimensions
// ==========================
pub const SCREEN_WIDTH: f32 = 640.0;
pub const SCREEN_HEIGHT: f32 = 480.0;
pub const PLAYER_SIZE_X: f32 = 34.0;
pub const PLAYER_SIZE_Y: f32 = 50.0;

// ==========================
// Maps and Backgrounds
// ==========================
pub const MAP_BOSS1: &str = "./img/mapa1.png";
pub const MAP_BOSS2: &str = "./img/mapa2.png";
pub const MAP_BOSS3: &str = "./img/mapa3.png";
pub const MAP_BOSS4: &str = "./img/mapa4.png";
pub const MAP_BOSS4_B: &str = "./img/mapa4_b.png";

// ==========================
// Player Images
// ==========================
pub const PLAYER_HEART_IMAGE: &str = "./img/heart.png";
pub const PLAYER_IMAGE_FRONT1: &str = "./img/player/front1.png";
pub const PLAYER_IMAGE_FRONT2: &str = "./img/player/front2.png";
pub const PLAYER_IMAGE_BACK1: &str = "./img/player/back1.png";
pub const PLAYER_IMAGE_BACK2: &str = "./img/player/back2.png";
pub const PLAYER_IMAGE_LEFT1: &str = "./img/player/left1.png";
pub const PLAYER_IMAGE_LEFT2: &str = "./img/player/left2.png";
pub const PLAYER_IMAGE_RIGHT1: &str = "./img/player/right1.png";
pub const PLAYER_IMAGE_RIGHT2: &str = "./img/player/right2.png";

// ==========================
// Boss Images
// ==========================
pub const BOSS1_IMAGE: &str = "./img/boss1/boss1.png";
pub const BOSS1_B_IMAGE: &str = "./img/boss1/boss1_b.png";
pub const BOSS2_IMAGE: &str = "./img/boss2/boss2.png";
pub const BOSS2_B_IMAGE: &str = "./img/boss2/boss2_b.png";
pub const BOSS3_IMAGE: &str = "./img/boss3/boss3.png";
pub const BOSS3_B_IMAGE: &str = "./img/boss3/boss3_b.png";
pub const BOSS4_LEFT_IMAGE: &str = "./img/boss4/boss4_left.png";
pub const BOSS4_RIGHT_IMAGE: &str = "./img/boss4/boss4_right.png";
pub const BOSS4_B_IMAGE: &str = "./img/boss4/boss4_b.png";
pub const PLAYER_IMAGE_TREE2: &str = "./img/arbol2.png";
pub const PLAYER_IMAGE_SNAKE2: &str = "./img/serpiente2.png";
pub const PLAYER_IMAGE_SNAKE2_SHOT: &str = "./img/serpiente2Bola.png";

// ==========================
// Shots and Weapons
// ==========================
pub const PLAYER_SHOT: &str = "./img/player_shot.png";
pub const BOSS1_SHOT: &str = "./img/boss1/boss1_shot.png";
pub const BOSS2_SHOT: &str = "./img/boss2/boss2_shot.png";
pub const BOSS2_B_SHOT: &str = "./img/boss2/boss2_b_shot.png";
pub const BOSS4_SHOT: &str = "./img/boss4/boss4_shot.png";
pub const BOSS3_WEAPON: &str = "./img/boss3/boss3_weapon.png";
pub const BOSS3_B_SHOT: &str = "./img/boss3/boss3_b_shot.png";
pub const PLAYER_SHOT_BOSS: &str = "./img/bolaSua.png";
pub const BOSS_IMAGE: &str = "./img/Boss.png";
pub const BOSS_RIGHT_IMAGE: &str = "./img/bossb.png";
pub const BOSS_RIGHT2_IMAGE: &str = "./img/Boss2.png";
pub const GAMEOVER_IMAGE: &str = "./img/gameOver.png";

// ==========================
// Sounds
// ==========================
pub const SOUND_WIN: &str = "./sound/BugleCall.wav";
pub const SOUND_LOSE: &str = "./sound/terminator.wav";
pub const END_SOUND_1: &str = "./sound/128NIGHT_01.wav";
pub const SOUND_BOSS1: &str = "./sound/Megalovania.wav";
pub const SOUND_BOSS2: &str = "./sound/Serpiente.wav";
pub const SOUND_BOSS3: &str = "./sound/Johnny.wav";
pub const SOUND_BOSS4A: &str = "./sound/Boss4.wav";
pub const SOUND_BOSS4B: &str = "./sound/FinalBoss.wav";
pub const SOUND_MENU: &str = "./sound/Menu.wav";
pub const SOUND_DIALOGUE: &str = "./sound/Dialogo.wav";
pub const SOUND_HIT: &str = "./sound/Golpe.wav";
pub const SOUND_GAMEOVER: &str = "./sound/gameOver.wav";

// ==========================
// Dialogue Images
// --------------------------
// Intro Dialogue
pub const DIALOGUE_INTRO_1: &str = "./img/intro/1.png";
pub const DIALOGUE_INTRO_2: &str = "./img/intro/2.png";
pub const DIALOGUE_INTRO_3: &str = "./img/intro/3.png";
pub const DIALOGUE_INTRO_4: &str = "./img/intro/4.png";
pub const DIALOGUE_INTRO_5: &str = "./img/intro/5.png";
pub const DIALOGUE_INTRO_6: &str = "./img/intro/6.png";
pub const DIALOGUE_INTRO_7: &str = "./img/intro/7.png";
pub const DIALOGUE_INTRO_8: &str = "./img/intro/8.png";
pub const DIALOGUE_INTRO_9: &str = "./img/intro/9.png";
pub const DIALOGUE_INTRO_10: &str = "./img/intro/10.png";
pub const DIALOGUE_INTRO_11: &str = "./img/intro/11.png";
pub const DIALOGUE_INTRO_12: &str = "./img/intro/12.png";

// Tree Dialogue 1 (Arbol1)
pub const DIALOGUE_ARBOL1_0: &str = "./img/arbol1/0.png";
pub const DIALOGUE_ARBOL1_1: &str = "./img/arbol1/1.png";
pub const DIALOGUE_ARBOL1_2: &str = "./img/arbol1/2.png";
pub const DIALOGUE_ARBOL1_3: &str = "./img/arbol1/3.png";
pub const DIALOGUE_ARBOL1_4: &str = "./img/arbol1/4.png";
pub const DIALOGUE_ARBOL1_5: &str = "./img/arbol1/5.png";
pub const DIALOGUE_ARBOL1_6: &str = "./img/arbol1/6.png";

// Snake Dialogue 1 (Sugea1)
pub const DIALOGUE_SUGEA1_0: &str = "./img/sugea1/0.png";
pub const DIALOGUE_SUGEA1_1: &str = "./img/sugea1/1.png";
pub const DIALOGUE_SUGEA1_2: &str = "./img/sugea1/2.png";
pub const DIALOGUE_SUGEA1_3: &str = "./img/sugea1/3.png";
pub const DIALOGUE_SUGEA1_4: &str = "./img/sugea1/4.png";
pub const DIALOGUE_SUGEA1_5: &str = "./img/sugea1/5.png";

// Octopus Dialogue 1 (Pulpo1)
pub const DIALOGUE_PULPO1_0: &str = "./img/pulpo1/0.png";
pub const DIALOGUE_PULPO1_1: &str = "./img/pulpo1/1.png";
pub const DIALOGUE_PULPO1_2: &str = "./img/pulpo1/2.png";
pub const DIALOGUE_PULPO1_3: &str = "./img/pulpo1/3.png";
pub const DIALOGUE_PULPO1_4: &str = "./img/pulpo1/4.png";
pub const DIALOGUE_PULPO1_5: &str = "./img/pulpo1/5.png";
pub const DIALOGUE_PULPO1_6: &str = "./img/pulpo1/6.png";
pub const DIALOGUE_PULPO1_7: &str = "./img/pulpo1/7.png";
pub const DIALOGUE_PULPO1_8: &str = "./img/pulpo1/8.png";

// Titan Dialogue 1 (Titan1)
pub const DIALOGUE_TITAN1_0: &str = "./img/titan1/0.png";
pub const DIALOGUE_TITAN1_1: &str = "./img/titan1/1.png";
pub const DIALOGUE_TITAN1_2: &str = "./img/titan1/2.png";
pub const DIALOGUE_TITAN1_3: &str = "./img/titan1/3.png";
pub const DIALOGUE_TITAN1_4: &str = "./img/titan1/4.png";
pub const DIALOGUE_TITAN1_5: &str = "./img/titan1/5.png";
pub const DIALOGUE_TITAN1_6: &str = "./img/titan1/6.png";
pub const DIALOGUE_TITAN1_7: &str = "./img/titan1/7.png";
pub const DIALOGUE_TITAN1_8: &str = "./img/titan1/8.png";

// Tree Dialogue 2 (Arbol2)
pub const DIALOGUE_ARBOL2_0: &str = "./img/arbol2/0.png";
pub const DIALOGUE_ARBOL2_1: &str = "./img/arbol2/1.png";
pub const DIALOGUE_ARBOL2_2: &str = "./img/arbol2/2.png";
pub const DIALOGUE_ARBOL2_3: &str = "./img/arbol2/3.png";

// Snake Dialogue 2 (Sugea2)
pub const DIALOGUE_SUGEA2_0: &str = "./img/sugea2/0.png";
pub const DIALOGUE_SUGEA2_1: &str = "./img/sugea2/1.png";
pub const DIALOGUE_SUGEA2_2: &str = "./img/sugea2/2.png";
pub const DIALOGUE_SUGEA2_3: &str = "./img/sugea2/3.png";
pub const DIALOGUE_SUGEA2_4: &str = "./img/sugea2/4.png";

// Octopus Dialogue 2A (Pulpo2A)
pub const DIALOGUE_PULPO2_0: &str = "./img/pulpo2/0.png";
pub const DIALOGUE_PULPO2_1: &str = "./img/pulpo2/1.png";
pub const DIALOGUE_PULPO2_2: &str = "./img/pulpo2/2.png";
pub const DIALOGUE_PULPO2_3: &str = "./img/pulpo2/3.png";
pub const DIALOGUE_PULPO2_4: &str = "./img/pulpo2/4.png";
pub const DIALOGUE_PULPO2_5: &str = "./img/pulpo2/5.png";

// Octopus Dialogue 2B (Pulpo2B)
pub const DIALOGUE_PULPO2_6: &str = "./img/pulpo2/6.png";
pub const DIALOGUE_PULPO2_7: &str = "./img/pulpo2/7.png";

// Titan Dialogue 2A (Titan2A)
pub const DIALOGUE_TITAN2_0: &str = "./img/titan2/0.png";
pub const DIALOGUE_TITAN2_1: &str = "./img/titan2/1.png";
pub const DIALOGUE_TITAN2_2: &str = "./img/titan2/2.png";
pub const DIALOGUE_TITAN2_3: &str = "./img/titan2/3.png";
pub const DIALOGUE_TITAN2_4: &str = "./img/titan2/4.png";
pub const DIALOGUE_TITAN2_5: &str = "./img/titan2/5.png";
pub const DIALOGUE_TITAN2_6: &str = "./img/titan2/6.png";
pub const DIALOGUE_TITAN2_7: &str = "./img/titan2/7.png";
pub const DIALOGUE_TITAN2_8: &str = "./img/titan2/8.png";
pub const DIALOGUE_TITAN2_9: &str = "./img/titan2/9.png";

// Titan Dialogue 2B (Titan2B)
pub const DIALOGUE_TITAN2_10: &str = "./img/titan2/10.png";
pub const DIALOGUE_TITAN2_11: &str = "./img/titan2/11.png";
pub const DIALOGUE_TITAN2_12: &str = "./img/titan2/12.png";
pub const DIALOGUE_TITAN2_13: &str = "./img/titan2/13.png";
pub const DIALOGUE_TITAN2_14: &str = "./img/titan2/14.png";
pub const DIALOGUE_TITAN2_15: &str = "./img/titan2/15.png";
pub const DIALOGUE_TITAN2_16: &str = "./img/titan2/16.png";

// ==========================
// Menu and Other UI Images
// ==========================
pub const MENU_START: &str = "./img/menu/menu_start.png";
pub const MENU_W: &str = "./img/menu/menu_w.png";
