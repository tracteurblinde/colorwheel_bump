use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Platform {}

#[derive(Bundle)]
pub struct PlatformBundle {
    #[bundle]
    pub sprite_bundle: SpriteBundle,
    pub platform: Platform,
    pub collider: Collider,
    pub rigid_body: RigidBody,

}

impl PlatformBundle {
    pub fn new(color: Color, position: Vec3, size: Vec2) -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                transform: Transform::from_translation(position),
                sprite: Sprite {
                    color,
                    custom_size: Some(size),
                    ..default()
                },
                ..default()
            },
            collider: Collider::cuboid(size.x / 2., size.y / 2.),
            ..default()
        }
    }
}

impl Default for PlatformBundle {
    fn default() -> Self {
        Self {
            platform: Platform {},
            sprite_bundle: SpriteBundle {
                transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
                sprite: Sprite {
                    color: Color::rgb(0.7, 0.0, 0.7),
                    custom_size: Some(Vec2::new(1., 1.)),
                    ..default()
                },
                ..default()
            },
            collider: Collider::cuboid(0.5, 0.5),
            rigid_body: RigidBody::Fixed,
        }
    }
}