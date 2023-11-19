use std::f32::consts::PI;

use macroquad::prelude::*;

use crate::Particle;

const PARTICLES_PER_POWER_POINT: usize = 20;
const PARTICLE_LIFE: f32 = 0.2;

pub struct Explosions {
    particles: Vec<Particle>,
}

impl Explosions {
    pub fn new() -> Self {
        Self {
            particles: Vec::new(),
        }
    }

    pub fn explode(&mut self, center: Vec2, power: u8) {
        for _ in 0..((power as usize) * PARTICLES_PER_POWER_POINT) {
            self.particles.push(Particle::new(
                center,
                rand::gen_range(0.0, PI * 2.0),
                PARTICLE_LIFE + rand::gen_range(0.0, 0.5),
            ));
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.particles
            .iter_mut()
            .for_each(|particle| particle.update(dt));
        self.particles.retain(|particle| particle.is_alive());
    }

    pub fn draw(&self) {
        self.particles.iter().for_each(|particle| particle.draw());
    }

    pub fn particles_count(&self) -> usize {
        self.particles.len()
    }
}
