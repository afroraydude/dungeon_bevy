use bevy::prelude::*;
use bevy::render::texture::ImageSettings;
use bevy::sprite::TextureAtlasBuilderResult;
use bevy_asset_loader::prelude::*;
use bevy_inspector_egui::{WorldInspectorPlugin, Inspectable};
use crate::components::camera::follow_player;
use crate::components::{BoxCollider, Collision, Unknown};
use crate::plugins::hello_plugin::HelloPlugin;
use crate::plugins::inspections::InspectionPlugin;
use crate::resources::assets::MyAssets;
use crate::systems::draw_begining;
use crate::systems::player::{animate_player, create_player, move_player};

mod components;
mod systems;
mod plugins;
mod resources;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum MyStates {
    AssetLoading,
    Next,
}

fn spawn_unknown(mut commands: Commands, assets: Res<MyAssets>) {
    commands.spawn_bundle(Unknown {
        name: crate::components::person::Name("Unknown".to_string()),
        collider: BoxCollider {
            width: 32.0,
            height: 32.0,
            layer: 0,
            offset: Vec2::new(0.0, 0.0),
        },
        collision: Collision {
            collisions: Vec::new(),
        },
        sprite: SpriteBundle {
            sprite: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            texture: assets.unknown.clone(),
            visibility: Default::default(),
            computed_visibility: Default::default()
        },
    });
}

fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest())
        .insert_resource(Msaa { samples: 1 })
        .add_plugins(DefaultPlugins)
        .add_loading_state(
            LoadingState::new(MyStates::AssetLoading)
                .continue_to_state(MyStates::Next)
                .with_collection::<MyAssets>(),
        )
        .add_state(MyStates::AssetLoading)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(InspectionPlugin)
        .add_system_set(
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
                .with_system(systems::box_colliders)
        )
        //.add_plugin(HelloPlugin)
        .run();
}
