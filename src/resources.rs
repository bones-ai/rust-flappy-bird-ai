use macroquad::prelude::*;
use once_cell::sync::OnceCell;

use crate::*;

pub static RESOURCES: OnceCell<Resources> = OnceCell::new();

pub struct Resources {
    pub bird_texture: Texture2D,
    pub pipe_texture: Texture2D,
    pub pipe_head_texture: Texture2D,
    pub background_texture: Texture2D,
}

pub async fn init_resources() {
    let resources = Resources::new().await;
    match RESOURCES.set(resources) {
        Ok(_) => println!("Resources init successfull"),
        Err(_) => panic!("Failed to load Resources"),
    };
}

impl Resources {
    pub async fn new() -> Self {
        let bird_texture = load_texture(BIRD_TEXTURE).await.unwrap();
        bird_texture.set_filter(FilterMode::Nearest);
        let pipe_head_texture = load_texture(PIPE_HEAD_TEXTURE).await.unwrap();
        pipe_head_texture.set_filter(FilterMode::Nearest);
        let pipe_texture = load_texture(PIPE_TEXTURE).await.unwrap();
        pipe_texture.set_filter(FilterMode::Nearest);
        let background_texture = load_texture(BACKGROUND_TEXTURE).await.unwrap();
        background_texture.set_filter(FilterMode::Nearest);

        Self {
            bird_texture,
            pipe_texture,
            pipe_head_texture,
            background_texture,
        }
    }
}
