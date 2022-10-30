use bevy::prelude::*;
use crate::components::player::{PlayerAnimationState};

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
    mut camera_query: Query<(&mut Transform, &Camera)>,
    time: ResMut<Time>,
    mut camera_timer: ResMut<CameraTimer>,
) {
    for (transform, _animation) in query.iter_mut() {
        for (mut camera_transform, _camera) in camera_query.iter_mut() {
            if camera_timer.0.tick(time.delta()).just_finished() {
                let target = transform.translation();

                let mut camera_x = camera_transform.translation.x;
                let mut camera_y = camera_transform.translation.y;

                let camera_x_target = target.x;
                let camera_y_target = target.y;

                // lerping between camera position and target position
                camera_x = lerp(camera_x, camera_x_target, 0.1);
                camera_y = lerp(camera_y, camera_y_target, 0.1);

                camera_transform.translation.x = camera_x;
                camera_transform.translation.y = camera_y;
            }
        }
    }
}