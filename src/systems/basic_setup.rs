//use bevy::prelude::*;

/*
pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // texture atlas for the player
    let texture_handle = asset_server.load("AnimationSheet_Character.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 9, 8);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn_bundle(SpriteSheetBundle {
        sprite: Default::default(),
        texture_atlas: texture_atlas_handle,
        transform: Transform::from_scale(Vec3::splat(2.0)),
        global_transform: Default::default(),
        visibility: Default::default(),
        computed_visibility: Default::default()
    });
}
*/