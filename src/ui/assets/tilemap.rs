use std::fs::File;
use std::io::prelude::*;

use serde::{Deserialize};
use macroquad::prelude::*;


#[derive(Clone, Copy, Deserialize, Debug)]
pub struct TileMapPosition {
    pub x: usize,
    pub y: usize,
    pub dim: usize,
}

#[derive(Clone)]
pub struct TileMap {
    pub tileset: Texture2D,
    pub tiles: Vec<TileMapPosition>
}

impl TileMap {
    pub async fn new(image: &str, info: &str) -> Self {
        let tileset = load_texture(image).await.unwrap();

        let mut file = File::open(info).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let tiles: Vec<TileMapPosition> = serde_json::from_str(&contents).unwrap();

        Self {
            tileset,
            tiles
        }
    }
}
