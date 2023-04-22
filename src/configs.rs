// Pipes
pub const PIPE_WIDTH: f32 = 75.0;
pub const PIPE_SPEED: f32 = 3.0;
pub const NUM_PIPES: usize = 10;
pub const MIN_VERTICAL_SPACE_BW_PIPES: f32 = 150.0;
pub const MIN_HORIZONTAL_SPACE_BW_PIPES: f32 = 450.0;
pub const MIN_PIPE_HEIGHT: f32 = 10.0;
pub const PIPES_START_X: f32 = 500.0;
pub const PIPE_HEAD_TEXTURE: &str = "./assets/pipe_head.png";
pub const PIPE_TEXTURE: &str = "./assets/pipe_up.png";

// Bird
pub const NUM_BIRDS: usize = 1000;
pub const BIRD_START_POS_X: f32 = 30.0;
pub const BIRD_GRAVITY: f32 = 0.3;
pub const BRAIN_MUTATION_RATE: f32 = 0.02;
pub const BRAIN_MUTATION_VARIATION: f32 = 0.2;
pub const BIRD_UP_FORCE: f32 = -5.0;
pub const BIRD_TEXTURE: &str = "./assets/bird.png";
pub const BIRD_TEXTURE_RESIZE: (f32, f32) = (30.0, 25.0);

// Others
pub const BACKGROUND_TEXTURE: &str = "./assets/background.png";
pub const PARALLAX_SPEED: f32 = 1.5;
