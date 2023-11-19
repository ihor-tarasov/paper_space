use std::f32::consts::PI;

use macroquad::prelude::*;

use crate::{Asteroid, Bullet, Explosions, Rocket, Ship};

const DISPLAY_SCALE: f32 = 1000.0;

pub struct Game {
    ship: Ship,
    bullets: Vec<Bullet>,
    explosions: Explosions,
    asteroids: Vec<Asteroid>,
    rockets: Vec<Rocket>,
    world_camera: Camera2D,
    is_game_over: bool,
}

impl Game {
    pub fn new() -> Self {
        Self {
            ship: Ship::new(),
            bullets: Vec::new(),
            explosions: Explosions::new(),
            asteroids: Vec::new(),
            rockets: Vec::new(),
            world_camera: Camera2D {
                rotation: 0.0,
                zoom: vec2(
                    1.0 / DISPLAY_SCALE,
                    (1.0 / DISPLAY_SCALE) * (screen_width() / screen_height()),
                ),
                target: vec2(0.0, 0.0),
                offset: vec2(0.0, 0.0),
                render_target: None,
                viewport: None,
            },
            is_game_over: false,
        }
    }

    fn generate_asteroid(&mut self) {
        if rand::gen_range(0, 100) == 0 {
            let angle = rand::gen_range(0.0, PI * 2.0);
            let position = self.ship.position() + Vec2::from_angle(angle) * DISPLAY_SCALE;
            let velocity = (self.ship.position() - position).normalize();
            self.asteroids.push(Asteroid::new(position, velocity));
        }
    }

    fn update_game_objects(&mut self, dt: f32) {
        self.asteroids.sort_by(|a, b| {
            a.position()
                .distance(self.ship.position())
                .total_cmp(&b.position().distance(self.ship.position()))
        });
        for i in 0..self.rockets.len() {
            let target = self
                .asteroids
                .get(i)
                .map(|asteroid| asteroid.position())
                .unwrap_or(
                    self.asteroids
                        .last()
                        .map(|asteroid| asteroid.position())
                        .unwrap_or(vec2(0.0, 0.0)),
                );
            self.rockets[i].update(target, dt);
        }

        self.ship.update(dt);
        self.bullets.iter_mut().for_each(|bullet| bullet.update(dt));
        self.asteroids
            .iter_mut()
            .for_each(|asteroid| asteroid.update(dt));
        self.explosions.update(dt);
    }

    fn process_collisions(&mut self) {
        let mut new_asteroids = Vec::new();
        for bullet in &mut self.bullets {
            for asteroid in &mut self.asteroids {
                if bullet.collision(asteroid.position(), asteroid.radius()) {
                    bullet.destroy();
                    asteroid.destroy(&mut new_asteroids);
                    self.explosions
                        .explode(asteroid.position(), asteroid.size());
                }
            }
        }

        let count = self.asteroids.len();
        for i in 0..count {
            for j in 0..count {
                if i != j {
                    if self.asteroids[i].collision(&self.asteroids[j]) {
                        self.asteroids[i].destroy(&mut new_asteroids);
                        self.asteroids[j].destroy(&mut new_asteroids);
                        self.explosions
                            .explode(self.asteroids[i].position(), self.asteroids[i].size());
                        self.explosions
                            .explode(self.asteroids[j].position(), self.asteroids[j].size());
                    }
                }
            }
            if self.ship.asteroid_collision(&self.asteroids[i]) {
                self.is_game_over = true;
            }
            self.rockets.iter_mut().for_each(|rocket| {
                if rocket.asteroid_collision(&self.asteroids[i]) {
                    self.asteroids[i].destroy(&mut new_asteroids);
                    self.explosions
                        .explode(self.asteroids[i].position(), self.asteroids[i].size());
                    rocket.destroy();
                }
            });
        }

        self.asteroids.extend(new_asteroids);
    }

    fn remove_objects(&mut self) {
        self.bullets.retain(|bullet| bullet.alive());
        self.asteroids.retain(|asteroid| asteroid.alive());
        self.rockets.retain(|rocket| rocket.is_alive());
    }

    fn update_camera(&mut self) {
        self.world_camera.zoom = vec2(
            1.0 / (DISPLAY_SCALE + self.ship.speed()),
            (1.0 / (DISPLAY_SCALE + self.ship.speed())) * (screen_width() / screen_height()),
        );
        self.world_camera.target = self.ship.position();
    }

    pub fn update(&mut self) {
        let dt = get_frame_time();

        self.generate_asteroid();

        if is_key_pressed(KeyCode::M) {
            self.bullets.push(self.ship.fire());
        }
        if is_key_pressed(KeyCode::N) {
            self.rockets.push(self.ship.launch_rocket());
        }

        self.update_game_objects(dt);
        self.process_collisions();
        self.remove_objects();
        self.update_camera();
    }

    pub fn draw(&self) {
        clear_background(LIGHTGRAY);
        set_camera(&self.world_camera);
        self.explosions.draw();
        self.ship.draw();
        self.bullets.iter().for_each(|bullet| bullet.draw());
        self.asteroids.iter().for_each(|asteroid| asteroid.draw());
        self.rockets.iter().for_each(|rocket| rocket.draw());
        set_default_camera();

        draw_text(
            format!("FPS: {}", get_fps()).as_str(),
            0.0,
            20.0,
            20.0,
            BLACK,
        );
        draw_text(
            format!("Bullets: {}", self.bullets.len()).as_str(),
            0.0,
            40.0,
            20.0,
            BLACK,
        );
        draw_text(
            format!("Asteroids: {}", self.asteroids.len()).as_str(),
            0.0,
            60.0,
            20.0,
            BLACK,
        );
        draw_text(
            format!(
                "Particles: {}",
                self.explosions.particles_count()
                    + self.ship.particles_count()
                    + self
                        .rockets
                        .iter()
                        .map(|rocket| rocket.particles_count())
                        .sum::<usize>()
            )
            .as_str(),
            0.0,
            80.0,
            20.0,
            BLACK,
        );
        draw_text(
            format!("Rockets: {}", self.rockets.len()).as_str(),
            0.0,
            100.0,
            20.0,
            BLACK,
        );
    }

    pub fn is_game_over(&self) -> bool {
        self.is_game_over
    }
}
