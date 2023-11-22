use std::f32::consts::PI;

use macroquad::prelude::*;

use crate::{asteroid::Asteroid, bullet::Bullet};

const HALF_SIZE: f32 = 5.0;
const PROPELLER_RADIUS: f32 = 5.0;
const PROPELLER_DISTANCE: f32 = 8.0;
const PROPELLER_ROTATION_SPEED: f32 = 40.0;
const MAX_TARGET_OFFSET_DISTANCE: f32 = 300.0;
const MOVE_ACCELERATION: f32 = 500.0;
const FAST_MAX_SPEED: f32 = 600.0;
const MAX_SPEED: f32 = 120.0;
const REGENERATE_TARGET_OFFESET_DISTANCE: f32 = 5.0;
const START_FAST_MOVING_DISTANCE: f32 = 600.0;
const STOP_FAST_MOVING_DISTANCE: f32 = 200.0;
const ROTATION_SPEED: f32 = 10.0;
const FIRE_DISTANCE: f32 = 500.0;
const RELOAD_TIME: f32 = 1.5;
const FULL_CHARGE_VALUE: f32 = 100.0;
const UNCHARGING_SPEED: f32 = 0.5;
const FAST_MOVE_UNCHARGING_SPEED: f32 = 2.5;
const FIRE_CHARGE_COST: f32 = 5.0;

fn generate_target_offset() -> Vec2 {
    let angle = rand::gen_range(0.0, PI * 2.0);
    Vec2::from_angle(angle) * MAX_TARGET_OFFSET_DISTANCE
}

pub struct Drone {
    position: Vec2,
    speed: f32,
    angle: f32,
    animation: f32,
    target_offet: Vec2,
    is_fast_moving: bool,
    reload_time: f32,
    charge: f32,
}

impl Drone {
    pub fn new(position: Vec2) -> Self {
        Self {
            position,
            animation: 0.0,
            speed: 0.0,
            angle: 0.0,
            target_offet: generate_target_offset(),
            is_fast_moving: false,
            reload_time: RELOAD_TIME,
            charge: FULL_CHARGE_VALUE,
        }
    }

    pub fn update(&mut self, dt: f32, target: Vec2) {
        if self.charge > 0.0 {
            if self.is_fast_moving {
                self.charge -= FAST_MOVE_UNCHARGING_SPEED * dt;
            } else {
                self.charge -= UNCHARGING_SPEED * dt;
            }
        }

        if self.reload_time > 0.0 {
            self.reload_time -= dt;
        }

        self.animation += dt * PROPELLER_ROTATION_SPEED;
        while self.animation > PI * 2.0 {
            self.animation -= PI * 2.0;
        }
        let target = target + self.target_offet;

        let angle = Vec2::from_angle(self.angle).angle_between(self.position - target);
        if angle > 0.0 {
            self.angle -= ROTATION_SPEED * dt;
        } else {
            self.angle += ROTATION_SPEED * dt;
        }
        while self.angle > PI * 2.0 {
            self.angle -= PI * 2.0;
        }

        let direction = target - self.position;

        let distance_squared = direction.length_squared();
        if distance_squared
            <= REGENERATE_TARGET_OFFESET_DISTANCE * REGENERATE_TARGET_OFFESET_DISTANCE
        {
            self.target_offet = generate_target_offset();
        }

        self.speed += MOVE_ACCELERATION * dt;

        if self.is_fast_moving {
            if distance_squared <= STOP_FAST_MOVING_DISTANCE * STOP_FAST_MOVING_DISTANCE {
                self.is_fast_moving = false;
            }
            if self.speed > FAST_MAX_SPEED {
                self.speed = FAST_MAX_SPEED;
            } else if self.speed < 0.0 {
                self.speed = 0.0;
            }
        } else {
            if distance_squared >= START_FAST_MOVING_DISTANCE * START_FAST_MOVING_DISTANCE {
                self.is_fast_moving = true;
            }
            if self.speed > MAX_SPEED {
                self.speed = MAX_SPEED;
            } else if self.speed < 0.0 {
                self.speed = 0.0;
            }
        }

        self.position += Vec2::from_angle(self.angle) * self.speed * dt;
    }

    fn draw_propeller(&self, x: f32, y: f32) {
        let point1 = Vec2::from_angle(self.animation) * PROPELLER_RADIUS;
        let point2 = Vec2::from_angle(self.animation + PI) * PROPELLER_RADIUS;
        draw_line(
            x + point1.x,
            y + point1.y,
            x + point2.x,
            y + point2.y,
            2.0,
            BLACK,
        );
        let point1 = Vec2::from_angle(self.animation + PI / 2.0) * PROPELLER_RADIUS;
        let point2 = Vec2::from_angle(self.animation + PI / 2.0 + PI) * PROPELLER_RADIUS;
        draw_line(
            x + point1.x,
            y + point1.y,
            x + point2.x,
            y + point2.y,
            2.0,
            BLACK,
        );
    }

    pub fn draw(&self) {
        let chars_count = if self.charge >= 10.0 { 3.0 } else { 2.0 };
        draw_text(format!("{}%", self.charge as i16).as_str(), self.position.x - (chars_count * 10.0 / 2.0), self.position.y - PROPELLER_DISTANCE * 2.0, 20.0, BLACK);
        draw_line(
            self.position.x - HALF_SIZE,
            self.position.y - HALF_SIZE,
            self.position.x + HALF_SIZE,
            self.position.y - HALF_SIZE,
            2.0,
            BLACK,
        );
        draw_line(
            self.position.x + HALF_SIZE,
            self.position.y - HALF_SIZE,
            self.position.x + HALF_SIZE,
            self.position.y + HALF_SIZE,
            2.0,
            BLACK,
        );
        draw_line(
            self.position.x + HALF_SIZE,
            self.position.y + HALF_SIZE,
            self.position.x - HALF_SIZE,
            self.position.y + HALF_SIZE,
            2.0,
            BLACK,
        );
        draw_line(
            self.position.x - HALF_SIZE,
            self.position.y + HALF_SIZE,
            self.position.x - HALF_SIZE,
            self.position.y - HALF_SIZE,
            2.0,
            BLACK,
        );
        self.draw_propeller(
            self.position.x - PROPELLER_DISTANCE,
            self.position.y - PROPELLER_DISTANCE,
        );
        self.draw_propeller(
            self.position.x + PROPELLER_DISTANCE,
            self.position.y - PROPELLER_DISTANCE,
        );
        self.draw_propeller(
            self.position.x - PROPELLER_DISTANCE,
            self.position.y + PROPELLER_DISTANCE,
        );
        self.draw_propeller(
            self.position.x + PROPELLER_DISTANCE,
            self.position.y + PROPELLER_DISTANCE,
        );
    }

    pub fn fire(&mut self, asteroids: &Vec<Asteroid>) -> Option<Bullet> {
        if self.is_fast_moving || self.reload_time > 0.0 {
            return None;
        }

        asteroids
            .iter()
            .min_by(|a, b| {
                a.position()
                    .distance(self.position)
                    .total_cmp(&b.position().distance(self.position))
            })
            .map(|asteroid| {
                (
                    asteroid.position(),
                    asteroid.position().distance(self.position),
                )
            })
            .and_then(|(position, distance)| {
                if distance < FIRE_DISTANCE {
                    let angle = Vec2::X.angle_between(position - self.position);
                    self.reload_time = RELOAD_TIME;
                    self.charge -= FIRE_CHARGE_COST;
                    Some(Bullet::new(
                        self.position + Vec2::from_angle(angle) * PROPELLER_RADIUS,
                        angle,
                    ))
                } else {
                    None
                }
            })
    }

    pub fn is_alive(&self) -> bool {
        self.charge > 0.0
    }

    pub fn position(&self) -> Vec2 {
        self.position
    }
}
