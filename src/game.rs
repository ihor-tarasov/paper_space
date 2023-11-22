use std::f32::consts::PI;

use macroquad::prelude::*;

use crate::{
    asteroid::Asteroid,
    bullet::Bullet,
    debug::{print_debug_info, Console},
    drone::Drone,
    explosions::Explosions,
    rockets::Rockets,
    ship::Ship,
};

const DISPLAY_SCALE: f32 = 1000.0;
const BULLET_RELOAD: f32 = 0.2;
const ROCKET_RELOAD: f32 = 0.5;
const DRONE_RELOAD: f32 = 1.0;
const ASTEROID_GENERATE_RATE: f32 = 1.0;
const DRONE_EXPLOSION_POWER: u8 = 4;
const ASTEROID_DESPAWN_DISTANCE: f32 = 10000.0;

pub struct Game {
    ship: Ship,
    bullets: Vec<Bullet>,
    bullet_reload: f32,
    explosions: Explosions,
    asteroids: Vec<Asteroid>,
    new_asteroids: Vec<Asteroid>,
    asteroid_generate_time: f32,
    rockets: Rockets,
    rocket_reload: f32,
    drones: Vec<Drone>,
    drone_reload: f32,
    world_camera: Camera2D,
    console: Console,
    is_game_over: bool,
}

impl Game {
    pub fn new() -> Self {
        Self {
            ship: Ship::new(),
            bullets: Vec::new(),
            bullet_reload: BULLET_RELOAD,
            explosions: Explosions::new(),
            asteroids: Vec::new(),
            new_asteroids: Vec::new(),
            asteroid_generate_time: ASTEROID_GENERATE_RATE,
            rockets: Rockets::new(),
            rocket_reload: ROCKET_RELOAD,
            drones: Vec::new(),
            drone_reload: DRONE_RELOAD,
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
            console: Console::new(),
            is_game_over: false,
        }
    }

    fn generate_asteroid(&mut self, dt: f32) {
        if self.asteroid_generate_time > 0.0 {
            self.asteroid_generate_time -= dt;
        } else {
            let angle = rand::gen_range(0.0, PI * 2.0);
            let position = self.ship.position() + Vec2::from_angle(angle) * DISPLAY_SCALE;
            let velocity = (self.ship.position() - position).normalize();
            self.asteroids.push(Asteroid::new(position, velocity));
            self.console.print(format!(
                "Generated asteroid ({}, {}).",
                position.x, position.y
            ));
            self.asteroid_generate_time = ASTEROID_GENERATE_RATE;
        }
    }

    fn update_game_objects(&mut self, dt: f32) {
        self.rockets.update(&mut self.asteroids, &self.ship, dt);
        self.ship.update(dt);
        self.bullets.iter_mut().for_each(|bullet| bullet.update(dt));
        self.asteroids
            .iter_mut()
            .for_each(|asteroid| asteroid.update(dt));
        self.explosions.update(dt);
        self.drones.iter_mut().for_each(|drone| {
            if let Some(bullet) = drone.fire(&self.asteroids) {
                self.bullets.push(bullet);
            }
        });
        self.drones
            .iter_mut()
            .for_each(|drone| drone.update(dt, self.ship.position()));
    }

    fn process_collisions(&mut self, dt: f32) {
        for bullet in &mut self.bullets {
            for asteroid in &mut self.asteroids {
                if bullet.collision(asteroid.position(), asteroid.radius()) {
                    bullet.destroy();
                    asteroid.destroy(&mut self.new_asteroids);
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
                        self.asteroids[i].destroy(&mut self.new_asteroids);
                        self.asteroids[j].destroy(&mut self.new_asteroids);
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
            self.rockets.asteroid_collision(
                &mut self.asteroids[i],
                &mut self.new_asteroids,
                &mut self.explosions,
            );
            if self.asteroids[i].position().distance(self.ship.position())
                >= ASTEROID_DESPAWN_DISTANCE
            {
                self.asteroids[i].despawn();
                self.console.print(format!(
                    "Asteroid despawned ({}, {}).",
                    self.asteroids[i].position().x,
                    self.asteroids[i].position().y
                ));
            }
        }
        for i in 0..self.drones.len() {
            for j in 0..self.drones.len() {
                if i != j {
                    let position = self.drones[j].position();
                    self.drones[i].drone_drone_collision(position, dt);
                }
            }
        }

        self.asteroids.reserve(self.new_asteroids.len());
        while let Some(new_asteroid) = self.new_asteroids.pop() {
            self.asteroids.push(new_asteroid);
        }
    }

    fn remove_objects(&mut self) {
        self.bullets.retain(|bullet| bullet.alive());
        self.asteroids.retain(|asteroid| asteroid.is_alive());
        self.drones.retain(|drone| {
            if drone.is_alive() {
                true
            } else {
                self.explosions
                    .explode(drone.position(), DRONE_EXPLOSION_POWER);
                self.console.print(format!(
                    "Drone destroyed ({}, {}).",
                    drone.position().x,
                    drone.position().y
                ));
                false
            }
        })
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

        self.generate_asteroid(dt);

        if self.bullet_reload > 0.0 {
            self.bullet_reload -= dt;
        } else {
            if is_key_down(KeyCode::M) {
                self.bullets.push(self.ship.fire());
                self.bullet_reload = BULLET_RELOAD;
            }
        }

        if self.rocket_reload > 0.0 {
            self.rocket_reload -= dt;
        } else {
            if is_key_down(KeyCode::N) {
                self.rockets.push(self.ship.launch_rocket());
                self.rocket_reload = ROCKET_RELOAD;
            }
        }

        if self.drone_reload > 0.0 {
            self.drone_reload -= dt;
        } else {
            if is_key_down(KeyCode::B) {
                self.drones.push(self.ship.spawn_drone());
                self.drone_reload = DRONE_RELOAD;
            }
        }

        self.update_game_objects(dt);
        self.process_collisions(dt);
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
        self.rockets.draw();
        self.drones.iter().for_each(|drone| drone.draw());
        set_default_camera();

        draw_text("M - Fire", screen_width() - 10.0 * 8.0, 20.0, 20.0, BLACK);
        draw_rectangle(
            screen_width() - 100.0 / BULLET_RELOAD * self.bullet_reload,
            20.0,
            100.0 / BULLET_RELOAD * self.bullet_reload,
            10.0,
            BLACK,
        );
        draw_text(
            "N - Launch rocket",
            screen_width() - 10.0 * 17.0,
            40.0,
            20.0,
            BLACK,
        );
        draw_rectangle(
            screen_width() - 100.0 / ROCKET_RELOAD * self.rocket_reload,
            40.0,
            100.0 / ROCKET_RELOAD * self.rocket_reload,
            10.0,
            BLACK,
        );
        draw_text(
            "B - Spawn drone",
            screen_width() - 10.0 * 15.0,
            60.0,
            20.0,
            BLACK,
        );
        draw_rectangle(
            screen_width() - 100.0 / DRONE_RELOAD * self.drone_reload,
            60.0,
            100.0 / DRONE_RELOAD * self.drone_reload,
            10.0,
            BLACK,
        );

        print_debug_info(
            self.bullets.len(),
            self.asteroids.len(),
            self.explosions.particles_count()
                + self.ship.particles_count()
                + self.rockets.particles_count(),
            self.rockets.len(),
            self.drones.len(),
        );

        self.console.draw();
    }

    pub fn is_game_over(&self) -> bool {
        self.is_game_over
    }
}
