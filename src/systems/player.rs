use bevy::prelude::*;
use crate::components::{AnimationTimer, Health};
use crate::components::player::{PlayerAnimationState, PlayerAnimationStates, PlayerBundle, PlayerXp};
use crate::{BoxCollider, Collision, MyAssets};
use crate::systems::world_gen::get_center_of_world;

pub fn create_player (
    mut commands: Commands,
    assets: Res<MyAssets>,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    let texture_atlas_handle = texture_atlases.get_handle(&assets.player);

    // center of map
    let center_of_world = get_center_of_world();

    let transform = Transform::from_translation(Vec3::new(center_of_world.x, center_of_world.y, 1.0));

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
            transform: transform.clone(),
            global_transform: Default::default(),
            visibility: Default::default(),
            computed_visibility: Default::default(),
            ..Default::default()
        },
        collision: crate::components::Collision {
            collisions: Vec::new(),
        },
        collider: crate::components::BoxCollider {
            width: 20.0,
            height: 20.0,
            layer: 0,
            offset: Vec2::new(0.0, 0.0),
            scale: Vec2::new(1.0, 1.0),
            collider_type: crate::components::ColliderType::Solid,
        },
    };
    player.sprite.sprite.index = 0;
    commands.spawn_bundle(player).insert(AnimationTimer(Timer::from_seconds(0.2, true)));

    info!("Player spawned and setup")
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
Prevent movement if colliding with a solid object
Lastly, when going left, the player should flip the sprite
 */
pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut PlayerAnimationState, &Collision, &mut TextureAtlasSprite)>,
    mut entities: Query<(Entity, &BoxCollider, &Transform), Without<PlayerAnimationState>>,
    time: Res<Time>
) {
    for (mut transform, mut animation, collision, mut sprite) in query.iter_mut() {
        let collisions: Vec<u32> = collision.collisions.clone();
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

        // if the player is colliding with a solid object, don't move in the direction of the collision
        if collision.is_colliding() {
            for (e, collider, other_transform) in entities.iter() {
                if collisions.contains(&e.id()) {
                    if collider.collider_type == crate::components::ColliderType::Solid {
                        // if the player is colliding with a solid object, don't move in the direction of the collision
                        // they will be able to move in the other directions


                        if (transform.translation.x + direction.x) < other_transform.translation.x {
                            // if the player is moving towards the solid object, don't move
                            if direction.x > 0.0 {
                                direction.x = 0.0;
                            }
                        }
                        else if (transform.translation.x + direction.x) > other_transform.translation.x {
                            // if the player is moving towards the solid object, don't move
                            if direction.x < 0.0 {
                                direction.x = 0.0;
                            }
                        }

                        if (transform.translation.y + direction.y) < other_transform.translation.y {
                            // if the player is moving towards the solid object, don't move
                            if direction.y > 0.0 {
                                direction.y = 0.0;
                            }
                        }
                        else if (transform.translation.y + direction.y) > other_transform.translation.y {
                            // if the player is moving towards the solid object, don't move
                            if direction.y < 0.0 {
                                direction.y = 0.0;
                            }
                        }
                    }
                }
            }
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
            transform.translation += direction * 100.0 * time.delta_seconds();
            animation.0 = PlayerAnimationStates::Walk;
        } else {
            animation.0 = PlayerAnimationStates::Idle;
        }
    }
}