use bevy::prelude::*;
use crate::MyAssets;

pub mod people;
pub mod player;
mod basic_setup;

pub fn draw_begining(
    mut commands: Commands,
    assets: Res<MyAssets>,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    commands.spawn_bundle(Camera2dBundle::default()).insert(crate::components::camera::CameraTimer(Timer::from_seconds(0.01, true)));
}