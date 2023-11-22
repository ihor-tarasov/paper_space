use game::Game;
use macroquad::prelude::*;

mod asteroid;
mod bullet;
mod explosions;
mod game;
mod particle;
mod ship;
mod rocket;
mod rockets;
mod debug;
mod drone;
mod mine;

fn config() -> Conf {
    Conf {
        window_title: "Asteroids".to_string(),
        fullscreen: true,
        sample_count: 8,
        ..Default::default()
    }
}

#[macroquad::main(config)]
async fn main() {
    let mut game = Game::new();

    show_mouse(false);

    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        game.update();
        game.draw();

        if game.is_game_over() {
            game = Game::new();
        }

        next_frame().await
    }
}
