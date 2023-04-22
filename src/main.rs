use flappy_ai::resources::init_resources;
use flappy_ai::simulation::Statistics;
use macroquad::prelude::*;

use flappy_ai::editor::Editor;
use flappy_ai::*;

#[macroquad::main("Flappy AI")]
async fn main() {
    rand::srand(macroquad::miniquad::date::now() as _);
    init_resources().await;

    let mut editor = Editor::new();
    let mut simulation = Simulation::new();
    let mut stats = Statistics::new();

    loop {
        clear_background(LIGHTGRAY);

        stats = simulation.update(&editor.settings).unwrap_or(stats);
        simulation.draw(&editor.settings);
        let gen_label = format!("Gen: {}", stats.generation_count);
        draw_text(&gen_label, screen_width() / 2.0 - 90.0, 150.0, 40.0, WHITE);

        editor.update();
        editor.draw(&stats);

        // Restart simulation
        if editor.settings.is_restart {
            editor.settings.is_restart = false;
            simulation = Simulation::new();
        }
        // Render skip
        if editor.settings.is_frame_skip {
            for _ in 0..10 {
                stats = simulation.update(&editor.settings).unwrap_or(stats);
            }
        }

        if is_key_pressed(KeyCode::Q) {
            break;
        }
        next_frame().await
    }
}
