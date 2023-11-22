use std::collections::VecDeque;

use macroquad::prelude::*;

const TEXT_SIZE: f32 = 20.0;
const CONSOLE_MAX_LINES: usize = 10;
const RELOAD_BAR_SIZE: f32 = 100.0;

pub fn debug_draw_text(text: &str, index: f32) {
    draw_text(
        text,
        0.0,
        screen_height() - index * TEXT_SIZE,
        TEXT_SIZE,
        BLACK,
    );
}

pub fn draw_info(text: &str, y: f32, reload: f32, reload_max: f32) {
    draw_text(text, screen_width() - TEXT_SIZE / 2.0 * text.len() as f32, y * TEXT_SIZE, TEXT_SIZE, BLACK);
    draw_rectangle(
        screen_width() - RELOAD_BAR_SIZE / reload_max * reload,
        TEXT_SIZE * y,
        RELOAD_BAR_SIZE / reload_max * reload,
        TEXT_SIZE / 2.0,
        BLACK,
    );
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
