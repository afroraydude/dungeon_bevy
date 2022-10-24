use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

pub mod person;
pub mod player;
pub mod camera;

#[derive(Component)]
pub struct AnimationTimer(pub Timer);

#[derive(Inspectable, Component)]
pub struct Health {
    pub(crate) hp: f32,
    pub(crate) armor: f32,
}