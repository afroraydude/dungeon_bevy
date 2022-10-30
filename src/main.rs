use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::log::LogSettings;
use bevy::prelude::*;
use bevy::render::texture::ImageSettings;
use bevy_asset_loader::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_inspector_egui::{WorldInspectorPlugin};

use plugins::dungeon_gen::DungeonGenPlugin;
use plugins::game_state_plugin::GameStatePlugin;
use plugins::main_menu::MainMenuPlugin;
use resources::MyStates;

use crate::components::{BoxCollider, Collision, LoadingText, Unknown};
use crate::plugins::inspections::InspectionPlugin;
use crate::resources::assets::MyAssets;

mod components;
mod systems;
mod plugins;
mod resources;
//mod functions;

fn spawn_unknown(mut commands: Commands, assets: Res<MyAssets>) {
    commands.spawn_bundle(Unknown {
        name: crate::components::person::Name("Unknown".to_string()),
        collider: BoxCollider {
            width: 32.0,
            height: 32.0,
            layer: 0,
            offset: Vec2::new(0.0, 0.0),
            scale: Vec2::new(1.0, 1.0),
            collider_type: crate::components::ColliderType::Solid,
        },
        collision: Collision {
            collisions: Vec::new(),
        },
        sprite: SpriteBundle {
            sprite: Default::default(),
            transform: Transform::from_xyz(64.0, 64.0, 1.0),
            global_transform: Default::default(),
            texture: assets.unknown.clone(),
            visibility: Default::default(),
            computed_visibility: Default::default(),
        },
    });

    info!("Test object generated")
}

fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest())
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(LogSettings {
            filter: "info,wgpu_core=warn,wgpu_hal=warn,bevygame=debug".into(),
            level: bevy::log::Level::DEBUG,
        })
        
        .add_plugins(DefaultPlugins)
        .add_plugin(TilemapPlugin)
        .add_loading_state(
            LoadingState::new(MyStates::AssetLoading)
                .continue_to_state(MyStates::DungeonGeneration)
                .with_collection::<MyAssets>(),
        )
        .add_state(MyStates::AssetLoading)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(InspectionPlugin)
        .add_plugin(GameStatePlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(DungeonGenPlugin)
        .add_plugin(MainMenuPlugin)
        //.add_plugin(WorldGenerationPlugin)
        //.add_plugin(FrameTimeDiagnosticsPlugin::default())
        .run();
}
