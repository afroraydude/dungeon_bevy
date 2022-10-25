use bevy::prelude::*;

use crate::{systems::{draw_begining, player::{create_player, move_player, animate_player}}, spawn_unknown, resources::MyStates, components::camera::follow_player};

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(MyStates::Next)
                .with_system(draw_begining)
                .with_system(create_player)
                .with_system(spawn_unknown),
        )
        .add_system_set(
            SystemSet::on_update(MyStates::Next)
                .with_system(move_player)
                .with_system(animate_player)
                .with_system(follow_player)
                .with_system(crate::systems::box_colliders),
        );
    }
}
