use bevy::prelude::*;

use crate::{systems::{draw_begining, player::{create_player, move_player, animate_player}}, spawn_unknown, resources::MyStates, components::camera::follow_player};
use crate::components::camera::CameraTimer;
use crate::resources::RenderTimer;
use crate::systems::world_gen::{despawn_outofrange_chunks, spawn_chunks_around_camera};

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraTimer(Timer::from_seconds(0.01, true)));
        app.insert_resource(RenderTimer(Timer::from_seconds(0.2, true)));
        app.add_system_set(
            SystemSet::on_enter(MyStates::Game)
                //.with_system(draw_begining)
                .with_system(create_player)
                .with_system(spawn_unknown),
        )
        .add_system_set(
            SystemSet::on_update(MyStates::Game)
                .with_system(move_player)
                .with_system(animate_player)
                .with_system(follow_player)
                .with_system(crate::systems::box_colliders)
                .with_system(spawn_chunks_around_camera)
                .with_system(despawn_outofrange_chunks)
        );
    }
}
