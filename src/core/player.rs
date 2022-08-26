use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct LocalPlayerHandle(pub usize);

#[derive(Component)]
pub struct Player {
    pub handle: usize,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub rollback: bevy_ggrs::Rollback,
    #[bundle]
    pub sprite_bundle: SpriteBundle,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub restitution: Restitution,
}

impl PlayerBundle {
    pub fn new(
        handle: usize,
        color: Color,
        position: Vec3,
        rollback_id: u32,
    ) -> Self {
        Self {
            player: Player { handle },
            rollback: bevy_ggrs::Rollback::new(rollback_id),
            sprite_bundle: SpriteBundle {
                transform: Transform::from_translation(position),
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(1., 1.)),
                    ..default()
                },
                ..default()
            },
            ..default()
        }
    }
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            player: Player { handle: 0 },
            rollback: bevy_ggrs::Rollback::new(0),
            sprite_bundle: SpriteBundle {
                transform: Transform::from_translation(Vec3::new(0., 0., 100.)),
                sprite: Sprite {
                    color: Color::rgb(0.7, 0.0, 0.7),
                    custom_size: Some(Vec2::new(1., 1.)),
                    ..default()
                },
                ..default()
            },
            rigid_body: RigidBody::Dynamic,
            collider: Collider::cuboid(0.5, 0.5),
            restitution: Restitution::coefficient(0.7),
        }
    }
}