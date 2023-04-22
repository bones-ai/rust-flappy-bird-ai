use std::f32::consts::PI;

use macroquad::prelude::*;

use crate::game::Pipe;
use crate::nn::Net;
use crate::resources::RESOURCES;
use crate::*;

#[derive(Clone)]
pub struct Bird {
    pub score: f32,
    pub is_dead: bool,

    pos: f32,
    vel: f32,
    acc: f32,
    brain: Net,
}

impl Bird {
    pub fn new() -> Self {
        Self {
            pos: screen_height() / 2.0,
            vel: 0.0,
            acc: 0.0,
            brain: Net::new(vec![5, 8, 1]),
            score: 0.0,
            is_dead: false,
        }
    }

    pub fn with_brain(other: &Bird) -> Self {
        let mut new_bird = Bird::new();
        new_bird.brain = other.brain.clone();
        new_bird
    }

    pub fn draw(&self) {
        if self.is_dead {
            return;
        }

        let resources = RESOURCES.get().unwrap();
        let vel = vec2(self.vel, 1.0);
        let heading = vel.y.atan2(vel.x) + PI / 2.0;
        draw_texture_ex(
            resources.bird_texture,
            BIRD_START_POS_X - 15.0,
            self.pos - 15.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(BIRD_TEXTURE_RESIZE.into()),
                flip_x: true,
                flip_y: true,
                rotation: heading,
                ..Default::default()
            },
        );
    }

    pub fn update(&mut self, top_pipe: &Pipe, bottom_pipe: &Pipe) {
        if self.is_dead {
            return;
        }

        self.score += 1.0;
        self.acc += BIRD_GRAVITY;
        self.vel += self.acc;
        self.pos += self.vel;
        self.acc = 0.0;

        // Up-Down collision
        if self.pos >= screen_height() {
            self.mark_dead();
        }
        if self.pos <= 0.0 {
            self.mark_dead();
        }

        // Pipe collision
        if top_pipe.pos.x <= BIRD_START_POS_X && top_pipe.pos.x + PIPE_WIDTH >= BIRD_START_POS_X {
            // Top pipe
            if self.pos >= 0.0 && self.pos <= top_pipe.h {
                self.mark_dead();
            }
            // Bottom pipe
            if self.pos >= screen_height() - bottom_pipe.h && self.pos <= screen_height() {
                self.mark_dead();
            }
        }

        // Brain Inputs
        let out = self.brain.predict(&vec![
            self.pos as f64 / screen_height() as f64,
            top_pipe.h as f64 / screen_height() as f64,
            bottom_pipe.h as f64 / screen_height() as f64,
            top_pipe.pos.x as f64 / screen_width() as f64,
            self.vel as f64 / 10.0,
        ])[0];
        if out >= 0.5 {
            self.up_force();
        }
    }

    pub fn mutate(&mut self) {
        self.brain.mutate();
    }

    fn mark_dead(&mut self) {
        self.is_dead = true;
    }

    fn up_force(&mut self) {
        self.vel = 0.0;
        self.acc += BIRD_UP_FORCE;
    }
}
