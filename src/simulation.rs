use rand::distributions::WeightedIndex;
use rand::prelude::Distribution;
use rand::thread_rng;

use crate::agent::Bird;
use crate::editor::Settings;
use crate::game::Game;
use crate::*;

pub struct Simulation {
    game: Game,
    birds: Vec<Bird>,
    generation_count: u32,
}

pub struct Statistics {
    pub generation_count: u32,
    pub birds_alive: u32,
}

impl Simulation {
    pub fn new() -> Self {
        Self {
            game: Game::new(),
            birds: (0..NUM_BIRDS).map(|_| Bird::new()).collect(),
            generation_count: 0,
        }
    }

    pub fn update(&mut self, settings: &Settings) -> Option<Statistics> {
        if settings.is_pause {
            return None;
        }

        let (top_pipe, bottom_pipe) = self.game.update();
        self.birds
            .iter_mut()
            .for_each(|a| a.update(&top_pipe, &bottom_pipe));

        let dead_count = self.birds.iter().filter(|b| b.is_dead).count();
        if dead_count == NUM_BIRDS {
            self.birds = self.selection();
            self.game.reset();
            self.generation_count += 1;
        }

        Some(Statistics {
            birds_alive: (NUM_BIRDS - dead_count) as u32,
            generation_count: self.generation_count,
        })
    }

    pub fn draw(&self, settings: &Settings) {
        if !settings.is_draw {
            return;
        }

        self.game.draw();
        if settings.show_one_bird {
            let alive_birds: Vec<&Bird> = self.birds.iter().filter(|b| !b.is_dead).collect();
            match alive_birds.first() {
                Some(bird) => bird.draw(),
                None => {}
            }
            return;
        }

        self.birds.iter().for_each(|b| b.draw());
    }

    fn selection(&self) -> Vec<Bird> {
        let mut rng = thread_rng();
        let gene_pool = self.calc_fitness();
        let mut new_birds = Vec::new();

        for _ in 0..NUM_BIRDS {
            let rand_bird = self.birds[gene_pool.sample(&mut rng)].clone();
            let mut new_bird = Bird::with_brain(&rand_bird);
            new_bird.mutate();
            new_birds.push(new_bird);
        }

        new_birds
    }

    fn calc_fitness(&self) -> WeightedIndex<f32> {
        let mut max_fitness = 0.0;
        let mut weights = Vec::new();

        for b in self.birds.iter() {
            if b.score > max_fitness {
                max_fitness = b.score;
            }
            weights.push(b.score);
        }
        weights
            .iter_mut()
            .for_each(|i| *i = (*i / max_fitness) * 100.0);

        WeightedIndex::new(&weights).expect("Failed to generate gene pool")
    }
}

impl Statistics {
    pub fn new() -> Self {
        Statistics {
            generation_count: 0,
            birds_alive: 0,
        }
    }
}
