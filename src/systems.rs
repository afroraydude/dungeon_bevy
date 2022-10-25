use std::borrow::BorrowMut;
use bevy::prelude::*;
use crate::{BoxCollider, MyAssets, MyStates};
use crate::components::WorldTile;
use crate::functions::tile_to_world_pos;
use noise::{core::perlin::{perlin_2d, perlin_3d, perlin_4d}, Fbm, Perlin, permutationtable::PermutationTable, utils::*};

pub mod people;
pub mod player;
mod basic_setup;

pub fn draw_begining(
    mut commands: Commands,
    assets: Res<MyAssets>,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    commands.spawn_bundle(Camera2dBundle::default()).insert(crate::components::camera::CameraTimer(Timer::from_seconds(0.01, true)));
    info!("Assets loaded, camera setup")
}

/*
Get the collisions for each collider

If there is a collision, provide the entity that is colliding with the collider
 */
pub fn box_colliders(
    mut commands: Commands,
    mut query: Query<(&crate::components::BoxCollider, Entity, &mut crate::components::Collision, &Transform)>,
    mut query2: Query<(&crate::components::BoxCollider, Entity, &Transform)>,
) {
    for (box_collider, entity, mut collision, transform) in query.iter_mut() {
        let mut collisions: Vec<u32> = collision.collisions.clone();
        for (box_collider2, entity2, transform2) in query2.iter_mut() {
            if entity == entity2 {
                continue;
            }
            if box_collider.collides_with(&box_collider2, &transform, &transform2) {
                if !collisions.contains(&entity2.id()) {
                    collisions.push(entity2.id());
                    debug!("Collision between {:?} and {:?}", entity.id(), entity2.id());
                }
            } else if collisions.contains(&entity2.id()) {
                collisions.retain(|&x| x != entity2.id());
            }
        }
        collision.collisions = collisions;
    }
}

pub fn generate_world(
    mut commands: Commands,
    assets: Res<MyAssets>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut app_state: ResMut<State<MyStates>>
) {
    info!("Generating world");


    let fbm = Fbm::<Perlin>::default();

    debug!("Fbm loaded");

    let mut map = PlaneMapBuilder::<_, 2>::new(fbm)
        .set_size(1024, 1024)
        .set_x_bounds(0.0, 1.0)
        .set_y_bounds(0.0, 1.0)
        .build();

    debug!("Perlin map built");

    // world size: 4096x4096
    let mut world = vec![vec![0; 4096]; 4096];

    let grasses = texture_atlases.get_handle(&assets.grasses);
    let stones = texture_atlases.get_handle(&assets.stone_grounds);
    let walls = texture_atlases.get_handle(&assets.walls);

    for x in 0..1024 {
        for y in 0..1024 {
            let value = map.get_value(x, y);
            match value {
                0.0..=0.1 => world[x][y] = 1,
                0.1..=0.2 => world[x][y] = 2,
                0.2..=0.3 => world[x][y] = 3,
                0.3..=0.4 => world[x][y] = 4,
                0.4..=0.5 => world[x][y] = 5,
                0.5..=0.6 => world[x][y] = 6,
                0.6..=0.7 => world[x][y] = 7,
                0.7..=0.8 => world[x][y] = 8,
                0.8..=0.9 => world[x][y] = 9,
                0.9..=1.0 => world[x][y] = 10,
                _ => world[x][y] = 0,
            }

            debug!("{} {} => {}", x, y, world[x][y]);

            let tile = world[x][y];

            let atlas = match tile {
                0 => grasses.clone(),
                1 => grasses.clone(),
                2 => grasses.clone(),
                3 => grasses.clone(),
                4 => stones.clone(),
                5 => stones.clone(),
                6 => stones.clone(),
                7 => grasses.clone(),
                8 => grasses.clone(),
                9 => grasses.clone(),
                _ => walls.clone(),
            };

            commands.spawn_bundle(
                WorldTile {
                    sprite: SpriteSheetBundle {
                        texture_atlas: atlas,
                        sprite: TextureAtlasSprite {
                            index: tile,
                            ..Default::default()
                        },
                        transform: Transform::from_translation(tile_to_world_pos(x as i32, y as i32, 32, 1024)),
                        ..default()
                    }
                }
            );

            debug!("Spawned tile at {} {}", x, y);
        }
    }

    app_state.set(MyStates::Game).unwrap_or_else(|e| error!("Failed to set state: {}", e));
}