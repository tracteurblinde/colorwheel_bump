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
    pub velocity: Velocity,
    pub collider: Collider,
    pub restitution: Restitution,
    pub external_impulse: ExternalImpulse,
    pub gravity: GravityScale,
    pub collider_mass_properties: ColliderMassProperties
}

impl PlayerBundle {
    pub fn with_gravity(mut self, gravity: f32) -> Self {
        self.gravity = GravityScale(gravity);
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.sprite_bundle.sprite.color = color;
        self
    }

    pub fn with_position(mut self, position: Vec3) -> Self {
        self.sprite_bundle.transform = Transform::from_translation(position);
        self
    }

    pub fn with_id(mut self, id: u32) -> Self {
        self.player.handle = id as usize;
        self.rollback = bevy_ggrs::Rollback::new(id);
        self
    }

    pub fn with_size(mut self, size: Vec2) -> Self {
        self.sprite_bundle.sprite.custom_size = Some(size);
        self.collider = Collider::cuboid(size.x / 2., size.y / 2.);
        self
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
            velocity: Velocity::default(),
            collider: Collider::cuboid(0.5, 0.5),
            restitution: Restitution::coefficient(0.7),
            external_impulse: ExternalImpulse::default(),
            gravity: GravityScale::default(),
            collider_mass_properties: ColliderMassProperties::Density(1.0)
        }
    }
}
