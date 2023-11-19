use macroquad::prelude::*;

const SPEED: f32 = 500.0;
const RADIUS: f32 = 2.0;

pub struct Particle {
    position: Vec2,
    angle: f32,
    life: f32,
}

impl Particle {
    pub fn new(position: Vec2, angle: f32, life: f32) -> Self {
        Self {
            position,
            angle,
            life,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.position += Vec2::from_angle(self.angle) * SPEED * dt;
        if self.life > 0.0 {
            self.life -= dt;
        }
    }

    pub fn is_alive(&self) -> bool {
        self.life > 0.0
    }

    pub fn draw(&self) {
        draw_circle(self.position.x, self.position.y, RADIUS, BLACK);
    }
}
