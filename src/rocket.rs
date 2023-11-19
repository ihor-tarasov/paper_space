use std::f32::consts::PI;

use macroquad::prelude::*;

use crate::{asteroid::Asteroid, particle::Particle};

const HEIGHT: f32 = 20.0;
const SHOLDER: f32 = 18.0;
const START_SPEED: f32 = 300.0;
const MOVE_ACCELERATION: f32 = 800.0;
const MAX_SPEED: f32 = 2000.0;
const ROTATION_SPEED: f32 = 5.0;
const PARTICLE_ANGLE_DIFF: f32 = PI / 5.0;
const ACCELERATING_TIME: f32 = 0.7;

pub struct Rocket {
    position: Vec2,
    angle: f32,
    speed: f32,
    particles: Vec<Particle>,
    is_alive: bool,
    accelerating_time: f32,
}

impl Rocket {
    pub fn new(position: Vec2, angle: f32) -> Self {
        Self {
            position,
            angle,
            speed: START_SPEED,
            particles: vec![],
            is_alive: true,
            accelerating_time: ACCELERATING_TIME,
        }
    }

    fn update_particles(&mut self, dt: f32) {
        self.particles.retain(|particle| particle.is_alive());
        self.particles
            .iter_mut()
            .for_each(|particle| particle.update(dt));

        let particle_start = self.position + Vec2::from_angle(self.angle + PI) * HEIGHT / 2.0;
        let particle_angle =
            self.angle + PI + rand::gen_range(-PARTICLE_ANGLE_DIFF, PARTICLE_ANGLE_DIFF);
        self.particles.push(Particle::new(
            particle_start,
            particle_angle,
            0.1 + rand::gen_range(0.0, 0.1),
        ));
    }

    pub fn update(&mut self, target: Vec2, dt: f32) {
        if self.accelerating_time > 0.0 {
            self.accelerating_time -= dt;
        } else {
            let angle = Vec2::from_angle(self.angle).angle_between(self.position - target);
            if angle > 0.0 {
                self.angle -= ROTATION_SPEED * dt;
            } else {
                self.angle += ROTATION_SPEED * dt;
            }
            while self.angle > PI * 2.0 {
                self.angle -= PI * 2.0;
            }
        }

        self.speed += MOVE_ACCELERATION * dt;

        if self.speed > MAX_SPEED {
            self.speed = MAX_SPEED;
        } else if self.speed < 0.0 {
            self.speed = 0.0;
        }

        self.position += Vec2::from_angle(self.angle) * self.speed * dt;
        self.update_particles(dt);
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

    pub fn asteroid_collision(&self, asteroid: &Asteroid) -> bool {
        asteroid.position().distance(self.position) <= asteroid.radius() + HEIGHT
    }

    pub fn particles_count(&self) -> usize {
        self.particles.len()
    }

    pub fn is_alive(&self) -> bool {
        self.is_alive
    }

    pub fn destroy(&mut self) {
        self.is_alive = false;
    }
}
