use std::borrow::BorrowMut;
use bevy::prelude::*;
use crate::{BoxCollider, MyAssets, MyStates};
use crate::components::WorldTile;
use crate::functions::tile_to_world_pos;
use noise::{core::perlin::{perlin_2d, perlin_3d, perlin_4d}, Fbm, Perlin, permutationtable::PermutationTable, utils::*};
use bevy_ecs_tilemap::prelude::*;
use bevy::{math::Vec3Swizzles, prelude::*, render::texture::ImageSettings, utils::HashSet};

pub mod people;
pub mod player;
mod basic_setup;

pub fn draw_begining(
    mut commands: Commands,
    assets: Res<MyAssets>,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    //commands.spawn_bundle(Camera2dBundle::default()).insert(crate::components::camera::CameraTimer(Timer::from_seconds(0.01, true)));
    commands.spawn_bundle(Camera2dBundle::default());
    info!("Assets loaded, camera setup");
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

const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 32.0, y: 32.0 };
// For this example, don't choose too large a chunk size.
const CHUNK_SIZE: UVec2 = UVec2 { x: 4, y: 4 };
// Render chunk sizes are set to 4 render chunks per user specified chunk.
const RENDER_CHUNK_SIZE: UVec2 = UVec2 {
    x: CHUNK_SIZE.x * 2,
    y: CHUNK_SIZE.y * 2,
};

#[derive(Default, Debug)]
pub struct ChunkManager {
    pub spawned_chunks: HashSet<IVec2>,
}

pub fn generate_world(
    mut commands: Commands,
    assets: Res<MyAssets>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut app_state: ResMut<State<MyStates>>
) {
    info!("Generating world");

    // create default camera
    //commands.spawn_bundle(Camera2dBundle::default());

    let texture_handle: Handle<Image> = assets.grasses_raw.clone();
    let tilemap_size = TilemapSize { x: 32, y: 32 };
    let tilemap_entity = commands.spawn().id();
    let mut tile_storage = TileStorage::empty(tilemap_size);

    let fbm = Fbm::<Perlin>::default();

    debug!("Fbm loaded");

    let mut map = PlaneMapBuilder::<_, 2>::new(fbm)
        .set_size(tilemap_size.x as usize, tilemap_size.y as usize)
        .set_x_bounds(0.0, 1.0)
        .set_y_bounds(0.0, 1.0)
        .build();

    debug!("Perlin map built");

    let mut world = vec![vec![0; tilemap_size.y as usize]; tilemap_size.x as usize];

    for x in 0..tilemap_size.x {
        for y in 0..tilemap_size.y {
            let value = map.get_value(x as usize, y as usize);

            match value {
                0.0..=0.1 => {
                    world[x as usize][y as usize] = 0;
                },
                0.1..=0.2 => {
                    world[x as usize][y as usize] = 1;
                },
                0.2..=0.3 => {
                    world[x as usize][y as usize] = 2;
                },
                0.3..=0.4 => {
                    world[x as usize][y as usize] = 3;
                },
                0.4..=0.5 => {
                    world[x as usize][y as usize] = 4;
                },
                0.5..=0.6 => {
                    world[x as usize][y as usize] = 5;
                },
                0.6..=0.7 => {
                    world[x as usize][y as usize] = 6;
                },
                0.7..=0.8 => {
                    world[x as usize][y as usize] = 7;
                },
                0.8..=0.9 => {
                    world[x as usize][y as usize] = 8;
                },
                0.9..=1.0 => {
                    world[x as usize][y as usize] = 9;
                },
                _ => {
                    world[x as usize][y as usize] = 0;
                }
            }

            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn()
                .insert_bundle(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture: TileTexture(world[x as usize][y as usize] as u32),
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);

            debug!("Spawned tile at {} {}", x, y);
        }
    }

    let tile_size = TilemapTileSize { x: 32.0, y: 32.0 };
    let grid_size = tile_size.into();

    commands
        .entity(tilemap_entity)
        .insert_bundle(TilemapBundle {
            grid_size,
            size: tilemap_size,
            storage: tile_storage,
            texture: TilemapTexture::Single(texture_handle),
            tile_size,
            transform: get_tilemap_center_transform(&tilemap_size, &grid_size, 0.0),
            ..Default::default()
        });

    app_state.overwrite_set(MyStates::Game).unwrap_or_else(|e| error!("Failed to overwrite state: {:?}", e));
}

fn spawn_chunks_around_camera(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    camera_query: Query<&Transform, With<Camera>>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    for transform in camera_query.iter() {
        let camera_chunk_pos = camera_pos_to_chunk_pos(&transform.translation.xy());
        for y in (camera_chunk_pos.y - 2)..(camera_chunk_pos.y + 2) {
            for x in (camera_chunk_pos.x - 2)..(camera_chunk_pos.x + 2) {
                if !chunk_manager.spawned_chunks.contains(&IVec2::new(x, y)) {
                    chunk_manager.spawned_chunks.insert(IVec2::new(x, y));
                    spawn_chunk(&mut commands, &asset_server, IVec2::new(x, y));
                }
            }
        }
    }
}

fn despawn_outofrange_chunks(
    mut commands: Commands,
    camera_query: Query<&Transform, With<Camera>>,
    chunks_query: Query<(Entity, &Transform)>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    for camera_transform in camera_query.iter() {
        for (entity, chunk_transform) in chunks_query.iter() {
            let chunk_pos = chunk_transform.translation.xy();
            let distance = camera_transform.translation.xy().distance(chunk_pos);
            if distance > 320.0 {
                let x = (chunk_pos.x as f32 / (CHUNK_SIZE.x as f32 * TILE_SIZE.x)).floor() as i32;
                let y = (chunk_pos.y as f32 / (CHUNK_SIZE.y as f32 * TILE_SIZE.y)).floor() as i32;
                chunk_manager.spawned_chunks.remove(&IVec2::new(x, y));
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}