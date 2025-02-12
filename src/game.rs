// src/game.rs
use crate::dialogue::*;
use crate::types::GamePhase;
use crate::bosses::boss1::boss1;
use crate::bosses::boss2::boss2;

pub async fn run_game() {    
    loop {
        show_menu_dialogue().await;
        show_intro_dialogue().await;
        
        let mut state;

        state = GamePhase::Win;
        
        // Boss 1
        loop {
            show_tree_dialogue1().await;
            state = boss1().await;
            if state == GamePhase::Lose {
                show_game_over_dialogue().await;
            } else {
                break;
            }
        }

        // Boss 2
        loop {
            show_snake_dialogue1().await;
            state = boss2().await;
            if state == GamePhase::Lose {
                show_game_over_dialogue().await;
            } else {
                break;
            }
        }

        // Boss 3
        loop {
            show_octopus_dialogue1().await;
            //state = boss3().await;
            if state == GamePhase::Lose {
                show_game_over_dialogue().await;
            } else {
                break;
            }
        }
        
        // Boss 4
        loop {
            show_titan_dialogue1().await;
            //state = boss4().await;
            if state == GamePhase::Lose {
                show_game_over_dialogue().await;
            } else {
                break;
            }
        }

        // Boss 1_b
        loop {
            show_tree_dialogue2().await;
            //state = boss4().await;
            if state == GamePhase::Lose {
                show_game_over_dialogue().await;
            } else {
                break;
            }
        }

        // Boss 2_b
        loop {
            show_snake_dialogue2().await;
            //state = boss4().await;
            if state == GamePhase::Lose {
                show_game_over_dialogue().await;
            } else {
                break;
            }
        }

        // Boss 3_b
        loop {
            show_octopus_dialogue2a().await;
            //state = boss4().await;
            if state == GamePhase::Lose {
                show_game_over_dialogue().await;
            } else {
                break;
            }
        }
        show_octopus_dialogue2b().await;

        // Boss 4_b
        loop {
            show_titan_dialogue2a().await;
            //state = boss4().await;
            if state == GamePhase::Lose {
                show_game_over_dialogue().await;
            } else {
                break;
            }
        }
        show_titan_dialogue2b().await;
    }
}
