use bevy::prelude::*;
use crate::components::player::{PlayerAnimationState, PlayerXp};

#[derive(Component)]
pub struct CameraTimer(pub Timer);

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/*
Smooth follow player using timers and lerping between positions
Camera is a 2D camera
*/
pub fn follow_player(
    mut query: Query<(&GlobalTransform, &PlayerAnimationState)>,
    mut camera_query: Query<(&mut Transform, &mut CameraTimer)>,
    time: ResMut<Time>,
) {
    for (transform, animation) in query.iter_mut() {
        for (mut camera_transform, mut camera_timer) in camera_query.iter_mut() {
            if camera_timer.0.tick(time.delta()).just_finished() {
                let player_position = transform.translation();
                let player_x = player_position.x;
                let player_y = player_position.y;
                let camera_position = camera_transform.translation;
                // lerping between camera position and player position using custom lerp function
                let new_x = lerp(camera_position.x, player_x, 0.1);
                let new_y = lerp(camera_position.y, player_y, 0.1);
                camera_transform.translation = Vec3::new(new_x, new_y, 0.0);
            }
        }
    }
}