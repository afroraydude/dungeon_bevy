use bevy::prelude::*;

pub fn tile_to_world_pos(x: i32, y: i32, tile_size: u32, world_size: u32) -> Vec3 {
    // 0, 0 is the center of the world, so we need to offset it by half the world size
    let offset = world_size as f32 / 2.0;
    let x = x as f32 * tile_size as f32 - offset;
    let y = y as f32 * tile_size as f32 - offset;
    Vec3::new(x, y, 0.0)
}