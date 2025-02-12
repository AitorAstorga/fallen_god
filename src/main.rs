// src/main.rs
mod types;
mod assets;
mod dialogue;
mod game;
mod objects;
mod bosses;

use game::run_game;

#[macroquad::main("Fallen God")]
async fn main() {
    run_game().await;
}
