use std::f32::consts::PI;

use macroquad::prelude::*;

const MAX_MOVE_SPEED: f32 = 100.0;
const MAX_ROTATION_SPEED: f32 = 3.0;
const MIN_RADIUS: f32 = 25.0;
const MAX_ASTEROID_SIZE: u8 = 5;
const DRAW_RADIUS_MULTIPLIER: f32 = 1.3;

#[derive(Clone)]
pub struct Asteroid {
    position: Vec2,
    velocity: Vec2,
    angle: f32,
    rotation_speed: f32,
    size: u8,
    is_alive: bool,
}

impl Asteroid {
    pub fn new(position: Vec2, velocity: Vec2) -> Self {
        Self {
            position,
            velocity: velocity * MAX_MOVE_SPEED,
            angle: 0.0,
            rotation_speed: rand::gen_range(-MAX_ROTATION_SPEED, MAX_ROTATION_SPEED),
            size: rand::gen_range(1, MAX_ASTEROID_SIZE),
            is_alive: true,
        }
    }

    pub fn new_smaller(position: Vec2, velocity: Vec2, size: u8) -> Self {
        Self {
            position,
            velocity,
            angle: 0.0,
            rotation_speed: rand::gen_range(-MAX_ROTATION_SPEED, MAX_ROTATION_SPEED),
            size,
            is_alive: true,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.position += self.velocity * dt;
        self.angle += self.rotation_speed * dt;
    }

    pub fn is_alive(&self) -> bool {
        self.is_alive
    }

    pub fn draw(&self) {
        draw_poly_lines(
            self.position.x,
            self.position.y,
            self.size * 3,
            self.radius() * DRAW_RADIUS_MULTIPLIER,
            self.angle.to_degrees(),
            2.0,
            BLACK,
        );
    }

    pub fn position(&self) -> Vec2 {
        self.position
    }

    pub fn radius(&self) -> f32 {
        self.size as f32 * MIN_RADIUS
    }

    pub fn destroy(&mut self, new_asteroids: &mut Vec<Asteroid>) {
        if !self.is_alive {
            return;
        }
        self.is_alive = false;
        if self.size == 1 {
            return;
        }
        let count = self.size;
        let angle_offset = rand::gen_range(0.0, PI * 2.0);
        for i in 0..count {
            let angle = angle_offset + (PI * 2.0) / MAX_ASTEROID_SIZE as f32 * i as f32;
            let position =
                self.position + Vec2::from_angle(angle) * (count as f32 * MIN_RADIUS);
            let velocity = (position - self.position).normalize() * MAX_MOVE_SPEED;
            new_asteroids.push(Asteroid::new_smaller(position, velocity, 1));
        }
    }

    pub fn collision(&self, other: &Self) -> bool {
        self.position.distance(other.position) <= self.size as f32 * MIN_RADIUS * 2.0
    }

    pub fn size(&self) -> u8 {
        self.size
    }
}
