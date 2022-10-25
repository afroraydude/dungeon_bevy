use bevy_ecs_tilemap::prelude::*;
use bevy::{math::Vec3Swizzles, prelude::*, render::texture::ImageSettings, utils::HashSet};

pub mod assets;

use bevy::prelude::Timer;

pub struct GreetTimer(pub Timer);

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum MyStates {
    AssetLoading,
    MainMenu,
    WorldGeneration,
    Game,
}

pub const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 32.0, y: 32.0 };
// For this example, don't choose too large a chunk size.
pub const CHUNK_SIZE: UVec2 = UVec2 { x: 4, y: 4 };
// Render chunk sizes are set to 4 render chunks per user specified chunk.
pub const RENDER_CHUNK_SIZE: UVec2 = UVec2 {
    x: CHUNK_SIZE.x * 2,
    y: CHUNK_SIZE.y * 2,
};

pub const WORLD_SIZE: UVec2 = UVec2 {
    x: 256,
    y: 256,
};

pub struct WorldMap {
    pub map: Vec<Vec<u32>>,
}

impl Default for WorldMap {
    fn default() -> Self {
        let mut map = Vec::new();
        for _ in 0..WORLD_SIZE.x {
            let mut row = Vec::new();
            for _ in 0..WORLD_SIZE.y {
                row.push(0);
            }
            map.push(row);
        }
        Self { map }
    }
}