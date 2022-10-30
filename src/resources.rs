use bevy_ecs_tilemap::prelude::*;
use bevy::prelude::*;

pub mod assets;

use bevy::prelude::Timer;

pub struct GreetTimer(pub Timer);

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum MyStates {
    AssetLoading,
    MainMenu,
    DungeonGeneration,
    Game,
}

pub struct RenderTimer(pub Timer);

pub const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 32.0, y: 32.0 };
// For this example, don't choose too large a chunk size.
pub const CHUNK_SIZE: UVec2 = UVec2 { x: 8, y: 8 };
/*// Render chunk sizes are set to 4 render chunks per user specified chunk.
pub const RENDER_CHUNK_SIZE: UVec2 = UVec2 {
    x: CHUNK_SIZE.x * 2,
    y: CHUNK_SIZE.y * 2,
};*/

pub const RENDER_SIZE: UVec2 = UVec2 {
    x: 4,
    y: 4
};

pub const RENDER_CHUNK_SIZE: UVec2 = UVec2 {
    x: CHUNK_SIZE.x * RENDER_SIZE.x * TILE_SIZE.x as u32,
    y: CHUNK_SIZE.y * RENDER_SIZE.y * TILE_SIZE.y as u32,
};

pub const WORLD_SIZE: UVec2 = UVec2 {
    x: 128,
    y: 128,
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