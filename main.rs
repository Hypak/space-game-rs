#![allow(dead_code)]
// #![allow(unused_imports)]
#![allow(clippy::useless_format)]


use level::level_1::Level1;
use macroquad::prelude as mq;

mod game;
use game::Game;

mod base;
mod camera;
mod direction;
mod game_object;
mod ship;
mod spawn_regions;
mod team;
mod weapon;
mod level;

mod prelude;
mod keybinds;

use game_object::GameObject;

fn window_conf() -> mq::Conf {
    mq::Conf {
        window_title: "Untitled Space Game".to_string(),
        sample_count: 16,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut player_level = 0.0;
    loop {
        mq::next_frame().await;
        let level = Level1;
        let game = Game::new(level, player_level);
        if let Some(mut game) = game {
            loop {
                game.camera.zoom *= f32::powf(1.1, mq::mouse_wheel().1);
                game.update(mq::get_frame_time());
                mq::clear_background(mq::DARKPURPLE);
                game.draw();

                if game.gameover {
                    break;
                }
                let text = &format!("Player Level {}", player_level);
                mq::draw_text(text, 0.0, 48.0, 48.0, mq::LIME);
                let text = &format!("{} / {} Bases Collected", game.collected_base_count, game.bases.len());
                mq::draw_text(text, 0.0, 2.0 * 48.0, 48.0, mq::LIME);
                if GameObject::is_overlapping(&game.player.game_object, &game.home_base) {
                    let text = &format!("At Home Base");
                    mq::draw_text(text, 0.0, 3.0 * 48.0, 48.0, mq::LIME);
                    if game.collected_base_count as f32 > player_level {
                        let text = &format!("Press return to restart and level up to {}", game.collected_base_count);
                        mq::draw_text(text, 0.0, 4.0 * 48.0, 48.0, mq::LIME);
                        if mq::is_key_pressed(keybinds::RESTART_KEY) {
                            player_level = game.collected_base_count as f32;
                            break;
                        }
                    } else {
                        let text = &format!("Press return to restart");
                        mq::draw_text(text, 0.0, 4.0 * 48.0, 48.0, mq::LIME);
                        if mq::is_key_pressed(keybinds::RESTART_KEY) {
                            break;
                        }
                    }
                }


                mq::next_frame().await
            }
        } else {
            loop {
                mq::clear_background(mq::PURPLE);
                let text = "You win\nPress return to restart";
                mq::draw_text(text, 0.0, 48.0, 48.0, mq::LIME);
                if mq::is_key_down(mq::KeyCode::Enter) {
                    break;
                }
                mq::next_frame().await
            }
        }
    }
}
