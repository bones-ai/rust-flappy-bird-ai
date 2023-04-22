use egui_macroquad::egui;
use macroquad::prelude::*;

use crate::simulation::Statistics;

pub struct Settings {
    pub is_pause: bool,
    pub is_draw: bool,
    pub is_restart: bool,
    pub is_frame_skip: bool,
    pub is_show_egui: bool,
    pub show_one_bird: bool,
}

pub struct Editor {
    pub settings: Settings,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            is_pause: false,
            is_draw: true,
            is_restart: false,
            is_frame_skip: false,
            is_show_egui: true,
            show_one_bird: false,
        }
    }
}

impl Editor {
    pub fn new() -> Self {
        Self {
            settings: Settings::new(),
        }
    }

    pub fn update(&mut self) {
        // Handle keyboard input
        if is_key_pressed(KeyCode::Space) {
            self.settings.is_pause = !self.settings.is_pause;
        }
        if is_key_pressed(KeyCode::Escape) {
            self.settings.is_show_egui = !self.settings.is_show_egui;
        }
        if is_key_pressed(KeyCode::R) {
            self.settings.is_restart = true;
        }
    }

    pub fn draw(&mut self, stats: &Statistics) {
        if !self.settings.is_show_egui {
            return;
        }

        egui_macroquad::ui(|ctx| {
            egui::Window::new("No Title")
                .title_bar(false)
                .min_width(200.0)
                .default_pos(egui::pos2(screen_width(), screen_height()))
                .show(ctx, |ui| {
                    egui::CollapsingHeader::new("Stats")
                        .default_open(true)
                        .show(ui, |ui| {
                            ui.label(format!("FPS: {}", get_fps()));
                            ui.label(format!("Gen: {}", stats.generation_count));
                            ui.label(format!("Birds: {}", stats.birds_alive));
                        });

                    egui::CollapsingHeader::new("Options")
                        .default_open(true)
                        .show(ui, |ui| {
                            ui.checkbox(&mut self.settings.is_draw, "Draw");
                            ui.checkbox(&mut self.settings.show_one_bird, "Show One");
                            ui.checkbox(&mut self.settings.is_frame_skip, "Fast mode");
                        });

                    egui::CollapsingHeader::new("Controls")
                        .default_open(true)
                        .show(ui, |ui| {
                            ui.checkbox(&mut self.settings.is_pause, "Pause");
                            if ui.add(egui::Button::new("Restart")).clicked() {
                                self.settings.is_restart = true;
                            }
                        });
                });
        });
        egui_macroquad::draw();
    }
}
