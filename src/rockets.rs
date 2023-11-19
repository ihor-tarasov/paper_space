use crate::{rocket::Rocket, particle::Particle, asteroid::Asteroid, ship::Ship, explosions::Explosions};

use macroquad::prelude::*;

pub struct Rockets {
    rockets: Vec<Rocket>,
    particles: Vec<Particle>,
}

impl Rockets {
    pub fn new() -> Self {
        Self {
            rockets: Vec::new(),
            particles: Vec::new(),
        }
    }

    pub fn update(&mut self, asteroids: &mut Vec<Asteroid>, ship: &Ship, dt: f32) {
        self.rockets.retain(|rocket| rocket.is_alive());
        asteroids.sort_by(|a, b| {
            a.position()
                .distance(ship.position())
                .total_cmp(&b.position().distance(ship.position()))
        });
        for i in 0..self.rockets.len() {
            let target = asteroids
                .get(i)
                .map(|asteroid| asteroid.position())
                .unwrap_or(
                    asteroids
                        .last()
                        .map(|asteroid| asteroid.position())
                        .unwrap_or(vec2(0.0, 0.0)),
                );
            self.rockets[i].update(target, dt);
        }
        self.particles.retain(|particle| particle.is_alive());
        self.particles.iter_mut().for_each(|particle| particle.update(dt));
        self.rockets.iter().for_each(|rocket| self.particles.push(rocket.spawn_particle()));
    }

    pub fn asteroid_collision(&mut self, asteroid: &mut Asteroid, new_asteroids: &mut Vec<Asteroid>, explosions: &mut Explosions) {
        self.rockets.iter_mut().for_each(|rocket| {
            if rocket.asteroid_collision(asteroid) {
                asteroid.destroy(new_asteroids);
                explosions
                    .explode(asteroid.position(), asteroid.size());
                rocket.destroy();
            }
        });
    }

    pub fn push(&mut self, rocket: Rocket) {
        self.rockets.push(rocket);
    }

    pub fn draw(&self) {
        self.rockets.iter().for_each(|rocket| rocket.draw());
        self.particles.iter().for_each(|particle| particle.draw());
    }

    pub fn particles_count(&self) -> usize {
        self.particles.len()
    }

    pub fn len(&self) -> usize {
        self.rockets.len()
    }
}
