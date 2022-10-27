use std::borrow::{Borrow, BorrowMut};
use std::cmp::min;
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
use bevy_inspector_egui::egui::Shape::Vec;
use noise::utils::{NoiseMapBuilder, PlaneMapBuilder};
use crate::resources::{CHUNK_SIZE, RENDER_CHUNK_SIZE, RENDER_SIZE, RenderTimer, TILE_SIZE, WORLD_SIZE, WorldMap};

pub mod people;
pub mod player;
mod basic_setup;
pub mod world_gen;
pub mod dungeon_gen;

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
        let mut collisions: std::vec::Vec<u32> = collision.collisions.clone();
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

pub fn remove_loading_text(
    mut commands: Commands,
    mut query: Query<(Entity, &LoadingText)>,
) {
    for (entity, _) in query.iter_mut() {
        commands.entity(entity).despawn_recursive();
    }
}