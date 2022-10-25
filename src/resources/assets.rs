use bevy_asset_loader::prelude::*;
use bevy::prelude::*;

#[derive(AssetCollection)]
pub struct MyAssets {
    #[asset(texture_atlas(tile_size_x = 32., tile_size_y = 32., columns = 9, rows = 8))]
    #[asset(path = "AnimationSheet_Character.png")]
    pub player: Handle<TextureAtlas>,

    #[asset(path = "unknown.png")]
    pub unknown: Handle<Image>,

    #[asset(texture_atlas(tile_size_x = 32., tile_size_y = 32., columns = 8, rows = 8))]
    #[asset(path = "TX Tileset Grass.png")]
    pub grasses: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 32., tile_size_y = 32., columns = 8, rows = 8))]
    #[asset(path = "TX Tileset Stone Ground.png")]
    pub stone_grounds: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 32., tile_size_y = 32., columns = 8, rows = 8))]
    #[asset(path = "TX Tileset Wall.png")]
    pub walls: Handle<TextureAtlas>,
}