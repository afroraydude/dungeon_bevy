use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use crate::components::Health;

pub enum PlayerAnimationStates {
    Idle,
    Blink,
    Walk,
    Run,
    Jump,
    Duck,
}

#[derive(Component)]
pub struct PlayerAnimationState(pub PlayerAnimationStates);

#[derive(Inspectable, Component)]
pub struct PlayerXp(pub u32);

#[derive(Bundle)]
pub struct PlayerBundle {
    pub health: Health,
    pub xp: PlayerXp,
    pub name: crate::components::person::Name,
    pub animation: PlayerAnimationState,
    #[bundle]
    pub sprite: SpriteSheetBundle
}
