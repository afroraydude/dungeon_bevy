use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

pub mod person;
pub mod player;
pub mod camera;

#[derive(Component)]
pub struct AnimationTimer(pub Timer);

#[derive(Inspectable, Component)]
pub struct Health {
    pub(crate) hp: f32,
    pub(crate) armor: f32,
}

/*
#[derive(Inspectable, Component)]
pub struct CircleCollider {
    pub(crate) radius: f32,
    pub layer: u32,
}
*/

#[derive(Inspectable, Component)]
pub struct BoxCollider {
    pub(crate) width: f32,
    pub(crate) height: f32,
    pub layer: u32,
    pub offset: Vec2,
}

impl BoxCollider {
    pub fn collides_with(&self, other: &BoxCollider, transform: &Transform, other_transform: &Transform) -> bool {
        let x = transform.translation.x + self.offset.x;
        let y = transform.translation.y + self.offset.y;
        let other_x = other_transform.translation.x + other.offset.x;
        let other_y = other_transform.translation.y + other.offset.y;
        let half_width = self.width / 2.0;
        let half_height = self.height / 2.0;
        let other_half_width = other.width / 2.0;
        let other_half_height = other.height / 2.0;
        let left = x - half_width;
        let right = x + half_width;
        let top = y + half_height;
        let bottom = y - half_height;
        let other_left = other_x - other_half_width;
        let other_right = other_x + other_half_width;
        let other_top = other_y + other_half_height;
        let other_bottom = other_y - other_half_height;
        if left < other_right && right > other_left && top > other_bottom && bottom < other_top {
            return true;
        }
        false
    }
}

#[derive(Component, Inspectable)]
pub struct Collision {
    pub(crate) collisions: Vec<u32>,
}

impl FromWorld for Collision {
    fn from_world(world: &mut World) -> Self {
        Collision {
            collisions: Vec::new(),
        }
    }
}

impl Collision {
    pub fn is_colliding(&self) -> bool {
        self.collisions.len() > 0
    }
}

#[derive(Bundle)]
pub struct Unknown {
    pub(crate) name: crate::components::person::Name,
    pub collider: crate::components::BoxCollider,
    pub collision: crate::components::Collision,
    #[bundle]
    pub sprite: SpriteBundle
}