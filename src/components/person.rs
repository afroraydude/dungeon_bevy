use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable};

#[derive(Component)]
pub struct Person;

#[derive(Component, Inspectable)]
pub struct Name(pub(crate) String);