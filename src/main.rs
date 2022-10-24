use bevy::prelude::*;
use bevy::render::texture::ImageSettings;
use bevy::sprite::TextureAtlasBuilderResult;
use bevy_asset_loader::prelude::*;
use bevy_inspector_egui::{WorldInspectorPlugin, Inspectable};
use crate::plugins::hello_plugin::HelloPlugin;
use crate::resources::assets::MyAssets;

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

#[derive(Inspectable, Component)]
struct Health {
    hp: f32,
    armor: f32,
}

#[derive(Inspectable, Component)]
struct PlayerXp(u32);

#[derive(Bundle)]
struct PlayerBundle {
    health: Health,
    xp: PlayerXp,
    name: crate::components::person::Name,

    #[bundle]
    sprite: SpriteSheetBundle
}

fn draw(
    mut commands: Commands,
    assets: Res<MyAssets>,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands
        .spawn_bundle(SpriteSheetBundle {
            transform: Transform::from_scale(Vec3::splat(2.0)),
            texture_atlas: assets.player.clone(),
            ..default()
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
        .add_system_set(
            SystemSet::on_enter(MyStates::Next)
                .with_system(draw),
        )
        //.add_plugin(HelloPlugin)
        .run();
}
