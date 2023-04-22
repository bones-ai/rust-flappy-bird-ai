use macroquad::prelude::*;
use macroquad::rand::gen_range;

use crate::resources::RESOURCES;
use crate::*;

pub struct Game {
    pipe_manager: PipeManager,
    parallax: ParallaxBackground,
}

#[derive(Clone)]
pub struct Pipe {
    pub pos: Vec2,
    pub h: f32,
}

pub struct PipeManager {
    pipes: Vec<Pipe>,
    num_removed: u32,
}

struct ParallaxBackground {
    p1: f32,
    p2: f32,
}

impl Pipe {
    fn new(pos: Vec2, h: f32) -> Self {
        Self { pos, h }
    }

    fn draw(&self) {
        let resources = RESOURCES.get().unwrap();
        let is_top_pipe = self.pos.y == 0.0;

        draw_texture_ex(
            resources.pipe_texture,
            self.pos.x,
            self.pos.y,
            WHITE,
            DrawTextureParams {
                flip_y: is_top_pipe,
                dest_size: Some((PIPE_WIDTH, self.h).into()),
                ..Default::default()
            },
        );

        // TODO Get to draw the entire pipe all at once!!
        if is_top_pipe {
            draw_texture_ex(
                resources.pipe_head_texture,
                self.pos.x - 2.0,
                self.pos.y + self.h - 12.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some((PIPE_WIDTH + 4.0, 24.0).into()),
                    ..Default::default()
                },
            );
        } else {
            draw_texture_ex(
                resources.pipe_head_texture,
                self.pos.x - 2.0,
                self.pos.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some((PIPE_WIDTH + 4.0, 24.0).into()),
                    ..Default::default()
                },
            );
        }
    }

    fn update(&mut self) {
        self.pos.x -= PIPE_SPEED;
    }
}

impl ParallaxBackground {
    fn new() -> Self {
        Self {
            p1: 0.0,
            p2: screen_width(),
        }
    }

    fn update(&mut self) {
        self.p1 -= PARALLAX_SPEED;
        self.p2 -= PARALLAX_SPEED;

        if self.p1 <= -1.0 * screen_width() {
            self.p1 = screen_width() - 5.0;
        }
        if self.p2 <= -1.0 * screen_width() {
            self.p2 = screen_width() - 5.0;
        }
    }

    fn draw(&self) {
        let resources = RESOURCES.get().unwrap();
        draw_texture_ex(
            resources.background_texture,
            self.p1,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some((screen_width(), screen_height()).into()),
                ..Default::default()
            },
        );
        draw_texture_ex(
            resources.background_texture,
            self.p2,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some((screen_width(), screen_height()).into()),
                ..Default::default()
            },
        );
    }
}

impl PipeManager {
    fn new() -> Self {
        let mut x_pos = PIPES_START_X;
        let mut pm = Self {
            pipes: Vec::new(),
            num_removed: 0,
        };

        for _ in 0..NUM_PIPES {
            pm.add_pipe(x_pos);
            x_pos += MIN_HORIZONTAL_SPACE_BW_PIPES;
        }

        pm
    }

    fn update(&mut self) -> (Pipe, Pipe) {
        // Move pipes closer to bird
        self.pipes.iter_mut().for_each(|p| p.update());

        // Remove pipes beyond the screen
        self.pipes.retain(|p| p.pos.x >= -1.0 * PIPE_WIDTH);

        // Add new pipes
        let num_pipes = NUM_PIPES - (self.pipes.len() as f64 / 2.0) as usize;
        let x_pos = self.pipes.last().unwrap().pos.x + MIN_HORIZONTAL_SPACE_BW_PIPES;
        for _ in 0..num_pipes {
            self.add_pipe(x_pos);
            self.num_removed += 1;
        }

        self.get_nearest_pipes()
    }

    fn draw(&self) {
        let score = format!("Score: {}", self.num_removed);
        self.pipes.iter().for_each(|p| p.draw());
        draw_text(
            score.as_str(),
            screen_width() / 2.0 - 90.0,
            200.0,
            40.0,
            WHITE,
        );
    }

    fn get_nearest_pipes(&self) -> (Pipe, Pipe) {
        let top_pipe_height = self.pipes[0].clone();
        let bottom_pipe_height = self.pipes[1].clone();

        (top_pipe_height, bottom_pipe_height)
    }

    fn add_pipe(&mut self, x_pos: f32) {
        // Upper pipe
        let pipe_height = gen_range(
            MIN_PIPE_HEIGHT,
            screen_height() - MIN_PIPE_HEIGHT - MIN_VERTICAL_SPACE_BW_PIPES,
        );
        self.pipes.push(Pipe::new(vec2(x_pos, 0.0), pipe_height));

        // lower pipe
        let pipe_height = screen_height() - MIN_VERTICAL_SPACE_BW_PIPES - pipe_height;
        self.pipes.push(Pipe::new(
            vec2(x_pos, screen_height() - pipe_height),
            pipe_height,
        ));
    }
}

impl Game {
    pub fn new() -> Self {
        Self {
            pipe_manager: PipeManager::new(),
            parallax: ParallaxBackground::new(),
        }
    }

    pub fn update(&mut self) -> (Pipe, Pipe) {
        self.parallax.update();
        self.pipe_manager.update()
    }

    pub fn draw(&self) {
        self.parallax.draw();
        self.pipe_manager.draw();
    }

    pub fn reset(&mut self) {
        self.pipe_manager = PipeManager::new();
    }
}
