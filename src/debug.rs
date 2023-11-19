use std::collections::VecDeque;

use macroquad::prelude::*;

const TEXT_SIZE: f32 = 20.0;
const CONSOLE_MAX_LINES: usize = 10;

fn debug_draw_text(text: &str, index: f32) {
    draw_text(
        text,
        0.0,
        screen_height() - index * TEXT_SIZE,
        TEXT_SIZE,
        BLACK,
    );
}

pub fn print_debug_info(
    bullets_count: usize,
    asteroids_count: usize,
    particles_count: usize,
    rockets_count: usize,
) {
    debug_draw_text(format!("FPS: {}", get_fps()).as_str(), 0.0);
    debug_draw_text(format!("Bullets: {bullets_count}").as_str(), 1.0);
    debug_draw_text(format!("Asteroids: {asteroids_count}").as_str(), 2.0);
    debug_draw_text(format!("Particles: {particles_count}",).as_str(), 3.0);
    debug_draw_text(format!("Rockets: {rockets_count}").as_str(), 4.0);
}

pub struct Console {
    lines: VecDeque<String>,
}

impl Console {
    pub fn new() -> Self {
        Self {
            lines: VecDeque::new(),
        }
    }

    pub fn print(&mut self, text: String) {
        if self.lines.len() == CONSOLE_MAX_LINES {
            self.lines.pop_front();
        }
        self.lines.push_back(text);
    }

    pub fn draw(&self) {
        self.lines.iter().enumerate().for_each(|(index, line)| {
            draw_text(
                &line,
                0.0,
                TEXT_SIZE * index as f32 + TEXT_SIZE,
                TEXT_SIZE,
                BLACK,
            )
        });
    }
}
