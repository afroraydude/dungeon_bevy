use bevy::prelude::*;
use bevy_inspector_egui::{InspectorPlugin, RegisterInspectable};
use crate::Collision;

pub struct InspectionPlugin;

impl Plugin for InspectionPlugin {
    fn build(&self, app: &mut App) {
        app.register_inspectable::<Collision>();
    }
}