use bevy::prelude::*;
use crate::components::{AnimationTimer, Health};
use crate::components::player::{PlayerAnimationState, PlayerAnimationStates, PlayerBundle, PlayerXp};
use crate::MyAssets;

pub fn create_player (
    mut commands: Commands,
    assets: Res<MyAssets>,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    let texture_atlas_handle = texture_atlases.get_handle(&assets.player);
    let mut player = PlayerBundle {
        health: Health {
            hp: 100.0,
            armor: 0.0,
        },
        xp: PlayerXp(0),
        name: crate::components::person::Name("Player".to_string()),
        animation: PlayerAnimationState(PlayerAnimationStates::Idle),
        sprite: SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(2.0)),
            global_transform: Default::default(),
            visibility: Default::default(),
            computed_visibility: Default::default(),
            ..Default::default()
        }
    };
    player.sprite.sprite.index = 0;
    commands.spawn_bundle(player).insert(AnimationTimer(Timer::from_seconds(0.2, true)));
}

/*
Animate based on the timer and PlayerAnimationState value of the entity
different PlayerAnimation variants have different numbers of frames
we use the number of frames to determine which frame to show
Idle: frames 0-1 (2 frames)
Blink: frames 9-10 (2 frames)
Walk: frame 18-21 (4 frames)
Run: frame 27-34 (8 frames)
 */
pub fn animate_player (
    time: Res<Time>,
    mut query: Query<(&mut TextureAtlasSprite, &mut AnimationTimer, &PlayerAnimationState)>,
) {
    for (mut sprite, mut timer, animation) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            match animation.0 {
                PlayerAnimationStates::Idle => {
                    sprite.index = (sprite.index + 1) % 2;
                }
                PlayerAnimationStates::Blink => {
                    sprite.index = (sprite.index + 1) % 2 + 9;
                }
                PlayerAnimationStates::Walk => {
                    sprite.index = (sprite.index + 1) % 4 + 18;
                }
                PlayerAnimationStates::Run => {
                    sprite.index = (sprite.index + 1) % 8 + 27;
                }
                PlayerAnimationStates::Jump => {
                    sprite.index = 35;
                }
                PlayerAnimationStates::Duck => {
                    sprite.index = 36;
                }
            }
        }
    }
}

/*
Using this for debugging
This will draw all the sprites in the texture atlas in a grid
8 rows and 9 columns of 32x32 sprites
start at the top left and go right then down
 */
fn draw_all_sprites(
    mut commands: Commands,
    my_assets: Res<MyAssets>,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    let texture_atlas_handle = texture_atlases.get_handle(&my_assets.player);
    let texture_atlas = texture_atlases.get(&texture_atlas_handle).unwrap();
    let mut x = 0.0;
    let mut y = 0.0;
    for i in 0..texture_atlas.textures.len() {
        if (i % 8) == 0 {
            x = 0.0;
            y += 32.0;
        }
        commands.spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite {
                index: i as usize,
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
            ..Default::default()
        });
        x += 32.0;
        if x > 32.0 * 8.0 {
            x = 0.0;
            y -= 32.0;
        }
    }
}

/*
Moving the player should get the keybaord input and move the player sprite
It should also change the animation state to walking if the player is moving
Lastly, when going left, the player should flip the sprite
 */
pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut PlayerAnimationState, &mut TextureAtlasSprite)>,
) {
    for (mut transform, mut animation, mut sprite) in query.iter_mut() {
        let mut direction = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::Left) {
            direction.x -= 1.0;
            // flip the sprite
            sprite.flip_x = true;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            direction.x += 1.0;
            // flip the sprite
            sprite.flip_x = false;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            direction.y -= 1.0;
        }
        if direction.length() > 0.0 {
            direction = direction.normalize();
            transform.translation += direction * 5.0;
            animation.0 = PlayerAnimationStates::Walk;
        } else {
            animation.0 = PlayerAnimationStates::Idle;
        }
    }
}