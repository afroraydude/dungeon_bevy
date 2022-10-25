use std::borrow::{Borrow, BorrowMut};
use bevy::prelude::*;
use crate::{BoxCollider, Collision, LoadingText, MyAssets, MyStates};
use crate::components::WorldTile;
use noise::{core::perlin::{perlin_2d, perlin_3d, perlin_4d}, Fbm, Perlin, permutationtable::PermutationTable};
use bevy_ecs_tilemap::prelude::*;
use bevy::{math::Vec3Swizzles, prelude::*, render::texture::ImageSettings, utils::HashSet};
use bevy::app::AppLabel;
use bevy::render::camera::{CameraProjection, CameraRenderGraph, DepthCalculation};
use bevy::render::primitives::Frustum;
use bevy::render::view::VisibleEntities;
use noise::utils::{NoiseMapBuilder, PlaneMapBuilder};
use crate::resources::{CHUNK_SIZE, TILE_SIZE, WORLD_SIZE, WorldMap};

pub mod people;
pub mod player;
mod basic_setup;

pub fn draw_begining(
    mut commands: Commands,
    assets: Res<MyAssets>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut app_state: ResMut<State<MyStates>>,
) {
    //commands.spawn_bundle(Camera2dBundle::default()).insert(crate::components::camera::CameraTimer(Timer::from_seconds(0.01, true)));

    commands.spawn_bundle(Camera2dBundle::default());
    info!("Assets loaded, camera setup");

    commands
        .spawn_bundle(
            // Create a TextBundle that has a Text with a single section.
            TextBundle::from_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                "Loading...".to_string(),
                TextStyle {
                    font: assets.font.clone(),
                    font_size: 100.0,
                    color: Color::WHITE,
                },
            ) // Set the alignment of the Text
                .with_text_alignment(TextAlignment::TOP_CENTER)
                // Set the style of the TextBundle itself.
                .with_style(Style {
                    align_self: AlignSelf::Center,
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        top: Val::Px(5.0),
                        left: Val::Px(15.0),
                        ..default()
                    },
                    ..default()
                }),
        )
        .insert(LoadingText);
    app_state.overwrite_set(MyStates::WorldGeneration).unwrap_or_else(|e| error!("Error: {}", e));
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

#[derive(Default, Debug)]
pub struct ChunkManager {
    pub spawned_chunks: HashSet<IVec2>,
}

fn tile_to_world_pos(tile_pos: TilePos, chunk_pos: IVec2) -> Vec2 {
    let mut x = (tile_pos.x as i32 + (chunk_pos.x * CHUNK_SIZE.x as i32)) as f32;
    let mut y = (tile_pos.y as i32 + (chunk_pos.y * CHUNK_SIZE.y as i32)) as f32;

    Vec2::new(x, y)
}

pub fn get_center_of_world() -> Vec2 {
    let mut x = (((WORLD_SIZE.x as i32) * TILE_SIZE.x as i32) / 2) as f32;
    let mut y = (((WORLD_SIZE.y as i32) * TILE_SIZE.y as i32) / 2) as f32;

    Vec2::new(x, y)
}

fn spawn_chunk(commands: &mut Commands, assets: &Res<MyAssets>, chunk_pos: IVec2, world_map: &Res<WorldMap>) {

    let tilemap_entity = commands.spawn().id();
    let mut tile_storage = TileStorage::empty(CHUNK_SIZE.into());
    // Spawn the elements of the tilemap.
    for x in 0..CHUNK_SIZE.x {
        for y in 0..CHUNK_SIZE.y {
            let tile_pos = TilePos { x, y };

            let tile_world_pos = tile_to_world_pos(tile_pos, chunk_pos);

            // dont spawn out of bounds tiles
            if tile_world_pos.x < 0.0 || tile_world_pos.y < 0.0 || tile_world_pos.x >= WORLD_SIZE.x as f32 || tile_world_pos.y >= WORLD_SIZE.y as f32 {
                continue;
            }

            let tile_entity = commands
                .spawn()
                .insert_bundle(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture: TileTexture(world_map.map[tile_world_pos.x as usize][tile_world_pos.y as usize]),
                    ..Default::default()
                })
                .id();
            commands.entity(tilemap_entity).add_child(tile_entity);
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let transform = Transform::from_translation(Vec3::new(
        chunk_pos.x as f32 * CHUNK_SIZE.x as f32 * TILE_SIZE.x,
        chunk_pos.y as f32 * CHUNK_SIZE.y as f32 * TILE_SIZE.y,
        0.0,
    ));
    let texture_handle: Handle<Image> = assets.grasses_raw.clone();
    commands
        .entity(tilemap_entity)
        .insert_bundle(TilemapBundle {
            grid_size: TILE_SIZE.into(),
            size: CHUNK_SIZE.into(),
            storage: tile_storage,
            texture: TilemapTexture::Single(texture_handle),
            tile_size: TILE_SIZE,
            transform,
            ..Default::default()
        });
}

pub fn generate_world(
    mut commands: Commands,
    assets: Res<MyAssets>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut app_state: ResMut<State<MyStates>>,
    mut world: ResMut<WorldMap>,
) {
    info!("Generating world");

    // create default camera
    //commands.spawn_bundle(Camera2dBundle::default());

    let fbm = Fbm::<Perlin>::default();

    debug!("Fbm loaded");

    let mut map = PlaneMapBuilder::<_, 2>::new(fbm)
        .set_size(WORLD_SIZE.x as usize, WORLD_SIZE.y as usize)
        .set_x_bounds(0.0, 1.0)
        .set_y_bounds(0.0, 1.0)
        .build();

    debug!("Perlin map built");

    for x in 0..WORLD_SIZE.x {
        for y in 0..WORLD_SIZE.y {
            let value = map.get_value(x as usize, y as usize);

            match value {
                0.0..=0.1 => {
                    world.map[x as usize][y as usize] = 0;
                },
                0.1..=0.2 => {
                    world.map[x as usize][y as usize] = 1;
                },
                0.2..=0.3 => {
                    world.map[x as usize][y as usize] = 2;
                },
                0.3..=0.4 => {
                    world.map[x as usize][y as usize] = 3;
                },
                0.4..=0.5 => {
                    world.map[x as usize][y as usize] = 4;
                },
                0.5..=0.6 => {
                    world.map[x as usize][y as usize] = 5;
                },
                0.6..=0.7 => {
                    world.map[x as usize][y as usize] = 6;
                },
                0.7..=0.8 => {
                    world.map[x as usize][y as usize] = 7;
                },
                0.8..=0.9 => {
                    world.map[x as usize][y as usize] = 8;
                },
                _ => {
                    world.map[x as usize][y as usize] = 0;
                }
            }
        }
    }

    debug!("World generated");

    app_state.overwrite_set(MyStates::Game).unwrap_or_else(|e| error!("Failed to overwrite state: {:?}", e));
}

fn camera_pos_to_chunk_pos(camera_pos: &Vec2) -> IVec2 {
    let camera_pos = camera_pos.as_ivec2();
    let chunk_size: IVec2 = IVec2::new(CHUNK_SIZE.x as i32, CHUNK_SIZE.y as i32);
    let tile_size: IVec2 = IVec2::new(TILE_SIZE.x as i32, TILE_SIZE.y as i32);
    camera_pos / (chunk_size * tile_size)
}

pub fn spawn_chunks_around_camera(
    mut commands: Commands,
    assets: Res<MyAssets>,
    camera_query: Query<&Transform, With<Camera>>,
    mut chunk_manager: ResMut<ChunkManager>,
    world_map: Res<WorldMap>,
) {
    for transform in camera_query.iter() {
        let camera_chunk_pos = camera_pos_to_chunk_pos(&transform.translation.xy());
        for y in (camera_chunk_pos.y - 2)..(camera_chunk_pos.y + 2) {
            for x in (camera_chunk_pos.x - 2)..(camera_chunk_pos.x + 2) {
                if !chunk_manager.spawned_chunks.contains(&IVec2::new(x, y)) {
                    chunk_manager.spawned_chunks.insert(IVec2::new(x, y));
                    spawn_chunk(&mut commands, &assets, IVec2::new(x, y), &world_map);
                }
            }
        }
    }
}





pub fn despawn_outofrange_chunks(
    mut commands: Commands,
    camera_query: Query<&Transform, With<Camera>>,
    chunks_query: Query<(Entity, &Transform), Without<Collision>>,
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