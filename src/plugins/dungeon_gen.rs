use bevy::prelude::*;

use crate::resources::MyStates;
use crate::systems::dungeon_gen::{gen_dungeon, gen_dungeon_stress_test};

pub struct DungeonGenPlugin;

impl Plugin for DungeonGenPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(MyStates::DungeonGeneration)
              .with_system(gen_dungeon_stress_test),
        );
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}