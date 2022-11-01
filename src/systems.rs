use bevy::prelude::*;
use crate::{BoxCollider, Collision, LoadingText, MyAssets, MyStates};
use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt};

pub mod people;
pub mod player;
mod basic_setup;
pub mod world_gen;
pub mod dungeon_gen;

pub fn draw_begining(
    mut commands: Commands,
    mut app_state: ResMut<State<MyStates>>,
) {
    //commands.spawn_bundle(Camera2dBundle::default()).insert(crate::components::camera::CameraTimer(Timer::from_seconds(0.01, true)));

    commands.spawn_bundle(Camera2dBundle::default());
    info!("Assets loaded, camera setup");


    app_state.overwrite_set(MyStates::DungeonGeneration).unwrap_or_else(|e| error!("Error: {}", e));
}

/*
Get the collisions for each collider

If there is a collision, provide the entity that is colliding with the collider
 */
pub fn box_colliders(
    mut query: Query<(&BoxCollider, Entity, &mut Collision, &Transform)>,
    mut query2: Query<(&BoxCollider, Entity, &Transform)>,
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

pub fn print_pc_data_to_debug() {
    // grabs the cpu, ram, and gpu data
    let mut sys = System::new_all();

    sys.refresh_all();

    debug!("System data:");
    debug!("System name:             {:?}", sys.name());
    debug!("System kernel version:   {:?}", sys.kernel_version());
    debug!("System OS version:       {:?}", sys.long_os_version());
    debug!("System host name:        {:?}", sys.host_name());
    debug!("NB CPUs: {}", sys.cpus().len());
    debug!("total memory: {} bytes", sys.total_memory());
}