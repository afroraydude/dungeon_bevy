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

#[derive(Inspectable)]
pub enum ColliderType {
    Trigger,
    Solid,
}

impl PartialEq for ColliderType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ColliderType::Trigger, ColliderType::Trigger) => true,
            (ColliderType::Solid, ColliderType::Solid) => true,
            _ => false,
        }
    }
}

#[derive(Inspectable, Component)]
pub struct BoxCollider {
    pub(crate) width: f32,
    pub(crate) height: f32,
    pub layer: u32,
    pub offset: Vec2,
    pub scale: Vec2,
    pub collider_type: ColliderType,
}

impl BoxCollider {
    pub fn collides_with(&self, other: &BoxCollider, transform: &Transform, other_transform: &Transform) -> bool {
        if self.layer != other.layer {
            return false;
        }

        let x = transform.translation.x + self.offset.x;
        let y = transform.translation.y + self.offset.y;
        let other_x = other_transform.translation.x + other.offset.x;
        let other_y = other_transform.translation.y + other.offset.y;
        let width = self.width * self.scale.x;
        let height = self.height * self.scale.y;
        let other_width = other.width * other.scale.x;
        let other_height = other.height * other.scale.y;
        x < other_x + other_width &&
            x + width > other_x &&
            y < other_y + other_height &&
            y + height > other_y
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


#[derive(Bundle)]
pub struct WorldTile {
    #[bundle]
    pub sprite: SpriteSheetBundle
}

#[derive(Component)]
pub struct LoadingText;
