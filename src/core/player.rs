use bevy::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};
use bevy_rapier2d::prelude::*;
use std::f32::consts::SQRT_2;

use super::crystal::CrystalColor;

pub const PLAYER_OUTLINE_WIDTH: f32 = 0.05;


#[derive(Component)]
pub struct Player {
    pub color: Option<CrystalColor>,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    #[bundle]
    pub shape_bundle: ShapeBundle,
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub collider: Collider,
    pub restitution: Restitution,
    pub external_impulse: ExternalImpulse,
    pub gravity: GravityScale,
    pub collider_mass_properties: ColliderMassProperties,
    pub locked_axes: LockedAxes,
    pub active_events: ActiveEvents,
}

impl PlayerBundle {
    pub fn from_shape(sides: usize, radius: f32) -> Self {
        let shape = shapes::RegularPolygon {
            sides,
            feature: shapes::RegularPolygonFeature::Radius(radius),
            ..shapes::RegularPolygon::default()
        };
        Self {
            shape_bundle: GeometryBuilder::build_as(
                &shape,
                DrawMode::Outlined {
                    fill_mode: bevy_prototype_lyon::prelude::FillMode::color(Color::CYAN),
                    outline_mode: StrokeMode::new(Color::PINK, PLAYER_OUTLINE_WIDTH),
                },
                Transform::from_translation(Vec3::new(0., 0., 100.)),
            ),
            collider: Collider::cuboid(radius / SQRT_2, radius / SQRT_2),
            ..default()
        }
    }

    pub fn with_gravity(mut self, gravity: f32) -> Self {
        self.gravity = GravityScale(gravity);
        self
    }

    pub fn with_color(mut self, fill_color: Color, outline_color: Color) -> Self {
        self.shape_bundle.mode = DrawMode::Outlined {
            fill_mode: bevy_prototype_lyon::prelude::FillMode::color(fill_color),
            outline_mode: StrokeMode::new(outline_color, PLAYER_OUTLINE_WIDTH),
        };
        self
    }

    pub fn with_position(mut self, x: f32, y: f32) -> Self {
        self.shape_bundle.transform = Transform::from_translation(Vec3::new(x, y, 100.));
        self
    }
}

impl Default for PlayerBundle {
    fn default() -> Self {
        let shape = shapes::RegularPolygon {
            sides: 4,
            feature: shapes::RegularPolygonFeature::Radius(1.),
            ..shapes::RegularPolygon::default()
        };
        Self {
            player: Player {
                color: None,
            },
            shape_bundle: GeometryBuilder::build_as(
                &shape,
                DrawMode::Outlined {
                    fill_mode: bevy_prototype_lyon::prelude::FillMode::color(Color::CYAN),
                    outline_mode: StrokeMode::new(Color::PINK, PLAYER_OUTLINE_WIDTH),
                },
                Transform::from_translation(Vec3::new(0., 0., 100.)),
            ),
            rigid_body: RigidBody::Dynamic,
            velocity: Velocity::default(),
            collider: Collider::cuboid(0.5, 0.5),
            restitution: Restitution::coefficient(0.7),
            external_impulse: ExternalImpulse::default(),
            gravity: GravityScale::default(),
            collider_mass_properties: ColliderMassProperties::Density(1.0),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            active_events: ActiveEvents::COLLISION_EVENTS,
        }
    }
}
