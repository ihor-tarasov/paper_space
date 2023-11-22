use macroquad::prelude::*;

use crate::asteroid::Asteroid;

const RADIUS: f32 = 15.0;

pub struct Mine {
    position: Vec2,
    is_alive: bool,
}

impl Mine {
    pub fn new(position: Vec2) -> Self {
        Self {
            position,
            is_alive: true,
        }
    }

    pub fn draw(&self) {
        draw_circle_lines(self.position.x, self.position.y, RADIUS, 2.0, BLACK);
    }

    pub fn asteroid_collision(&self, asteroid: &Asteroid) -> bool {
        asteroid.position().distance(self.position) <= asteroid.radius() + RADIUS
    }

    pub fn is_alive(&self) -> bool {
        self.is_alive
    }

    pub fn destroy(&mut self) {
        self.is_alive = false;
    }

    pub fn position(&self) -> Vec2 {
        self.position
    }
}
