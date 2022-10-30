use bevy::prelude::*;

use crate::resources::MyStates;


pub struct WorldGenerationPlugin;

impl Plugin for WorldGenerationPlugin {
    fn build(&self, app: &mut App) {
      app.insert_resource(crate::resources::WorldMap::default())
      .insert_resource(crate::systems::world_gen::ChunkManager::default())
      .add_system_set(
        SystemSet::on_enter(MyStates::DungeonGeneration)
            .with_system(crate::systems::world_gen::generate_world)
        );
    }
}