use bevy::prelude::Plugin;

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub struct TilemapDebugPlugin;

impl Plugin for TilemapDebugPlugin {
    fn build(&self, app: &mut App) {
      //app.add_system(tilemap_debug_system);
    }
}