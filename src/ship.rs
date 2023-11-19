use std::f32::consts::PI;

use macroquad::prelude::*;

use crate::{asteroid::Asteroid, bullet::Bullet, particle::Particle, rocket::Rocket};

const HEIGHT: f32 = 25.0;
const SHOLDER: f32 = 22.0;
const MAX_ROTATION_SPEED: f32 = 7.0;
const MOVE_ACCELERATION: f32 = 300.0;
const ROTATION_ACCELERATION: f32 = 14.0;
const MAX_SPEED: f32 = 300.0;
const MOVE_FRICTION: f32 = 100.0;
const ROTATION_FRICTION: f32 = 10.0;
const PARTICLE_ANGLE_DIFF: f32 = PI / 4.0;

pub struct Ship {
    position: Vec2,
    speed: f32,
    angle: f32,
    rotation_speed: f32,
    particles: Vec<Particle>,
}

impl Ship {
    pub fn new() -> Self {
        Self {
            position: vec2(0.0, 0.0),
            speed: 0.0,
            angle: -PI / 2.0,
            rotation_speed: 0.0,
            particles: vec![],
        }
    }

    fn handle_rotation_input(&mut self, dt: f32) {
        let mut pressed = false;
        if is_key_down(KeyCode::A) {
            pressed = true;
            self.rotation_speed -= ROTATION_ACCELERATION * dt;
            if self.rotation_speed < -MAX_ROTATION_SPEED {
                self.rotation_speed = -MAX_ROTATION_SPEED;
            }
        }
        if is_key_down(KeyCode::D) {
            pressed = true;
            self.rotation_speed += ROTATION_ACCELERATION * dt;
            if self.rotation_speed > MAX_ROTATION_SPEED {
                self.rotation_speed = MAX_ROTATION_SPEED;
            }
        }
        if !pressed {
            if self.rotation_speed > 0.0 {
                self.rotation_speed -= ROTATION_FRICTION * dt;
            } else {
                self.rotation_speed += ROTATION_FRICTION * dt;
            }
        }
    }

    fn handle_move_input(&mut self, dt: f32) -> bool {
        let acceleration;

        if is_key_down(KeyCode::W) {
            acceleration = MOVE_ACCELERATION;
        } else if is_key_down(KeyCode::S) {
            acceleration = -MOVE_ACCELERATION;
        } else {
            acceleration = -MOVE_FRICTION;
        }

        self.speed += acceleration * dt;

        if self.speed > MAX_SPEED {
            self.speed = MAX_SPEED;
        } else if self.speed < 0.0 {
            self.speed = 0.0;
        }

        acceleration > 0.0
    }

    fn apply_rotation_speed(&mut self, dt: f32) {
        self.angle += self.rotation_speed * dt;

        while self.angle > PI * 2.0 {
            self.angle -= PI * 2.0;
        }
    }

    fn appy_move(&mut self, dt: f32) {
        self.position += Vec2::from_angle(self.angle) * self.speed * dt;
    }

    fn update_particles(&mut self, dt: f32, accelerated: bool) {
        self.particles.retain(|particle| particle.is_alive());
        self.particles
            .iter_mut()
            .for_each(|particle| particle.update(dt));

        if accelerated {
            let particle_start = self.position + Vec2::from_angle(self.angle + PI) * HEIGHT / 2.0;
            let particle_angle =
                self.angle + PI + rand::gen_range(-PARTICLE_ANGLE_DIFF, PARTICLE_ANGLE_DIFF);
            self.particles.push(Particle::new(
                particle_start,
                particle_angle,
                0.1 + rand::gen_range(0.0, 0.1),
            ));
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.handle_rotation_input(dt);
        self.apply_rotation_speed(dt);
        let accelerated = self.handle_move_input(dt);
        self.appy_move(dt);
        self.update_particles(dt, accelerated);
    }

    pub fn draw(&self) {
        let left_sholder_angle = self.angle + PI / 2.0;
        let direction = Vec2::from_angle(self.angle) * HEIGHT / 2.0;
        let left_sholder_direction = Vec2::from_angle(left_sholder_angle) * SHOLDER / 2.0;

        draw_triangle_lines(
            self.position + direction,
            self.position - direction + left_sholder_direction,
            self.position - direction - left_sholder_direction,
            2.0,
            BLACK,
        );

        self.particles.iter().for_each(|particle| particle.draw());
    }

    pub fn fire(&self) -> Bullet {
        Bullet::new(
            self.position + Vec2::from_angle(self.angle) * HEIGHT / 2.0,
            self.angle,
        )
    }

    pub fn launch_rocket(&self) -> Rocket {
        Rocket::new(
            self.position + Vec2::from_angle(self.angle) * HEIGHT / 2.0,
            self.angle,
        )
    }

    pub fn position(&self) -> Vec2 {
        self.position
    }

    pub fn speed(&self) -> f32 {
        self.speed
    }

    pub fn particles_count(&self) -> usize {
        self.particles.len()
    }

    pub fn asteroid_collision(&self, asteroid: &Asteroid) -> bool {
        asteroid.position().distance(self.position) <= asteroid.radius() + HEIGHT
    }
}
