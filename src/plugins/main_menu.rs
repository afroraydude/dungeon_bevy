use bevy::prelude::*;

use crate::{systems::draw_begining, resources::MyStates};


pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
          SystemSet::on_enter(MyStates::MainMenu)

              .with_system(draw_begining)
        );
        app.add_system_set(
          SystemSet::on_exit(MyStates::DungeonGeneration)
              .with_system(crate::systems::remove_loading_text)
        );
    }
}