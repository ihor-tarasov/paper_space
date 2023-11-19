use macroquad::prelude::*;

const SPEED: f32 = 1000.0;
const RADIUS: f32 = 2.0;
const LIFETIME: f32 = 1.0;

pub struct Bullet {
    position: Vec2,
    angle: f32,
    life: f32,
}

impl Bullet {
    pub fn new(position: Vec2, angle: f32) -> Self {
        Self {
            position,
            angle,
            life: LIFETIME,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.position += Vec2::from_angle(self.angle) * SPEED * dt;
        self.life -= dt;
    }

    pub fn alive(&self) -> bool {
        self.life > 0.0
    }

    pub fn draw(&self) {
        draw_circle(self.position.x, self.position.y, RADIUS, BLACK);
    }

    pub fn collision(&self, position: Vec2, radius: f32) -> bool {
        position.distance(self.position) <= RADIUS + radius
    }

    pub fn destroy(&mut self) {
        self.life = -1.0;
    }
}
