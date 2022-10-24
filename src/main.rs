use bevy::prelude::*;
use bevy::render::texture::ImageSettings;
use bevy::sprite::TextureAtlasBuilderResult;
use bevy_asset_loader::prelude::*;
use bevy_inspector_egui::{WorldInspectorPlugin, Inspectable};
use crate::components::camera::follow_player;
use crate::plugins::hello_plugin::HelloPlugin;
use crate::resources::assets::MyAssets;
use crate::systems::draw_begining;
use crate::systems::player::{animate_player, create_player, move_player};

mod components;
mod entities;
mod systems;
mod plugins;
mod resources;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum MyStates {
    AssetLoading,
    Next,
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
        .add_system_set(
            SystemSet::on_enter(MyStates::Next)
                .with_system(draw_begining)
                .with_system(create_player),
        )
        .add_system_set(
            SystemSet::on_update(MyStates::Next)
                .with_system(move_player)
                .with_system(animate_player)
                .with_system(follow_player)
        )
        //.add_plugin(HelloPlugin)
        .run();
}
