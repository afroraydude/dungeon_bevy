use bevy::prelude::*;

use crate::{resources::MyStates, systems::dungeon_gen::gen_dungeon};

struct DungeonGenPlugin;

impl Plugin for DungeonGenPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(MyStates::Game)
              .with_system(gen_dungeon),
        );
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}