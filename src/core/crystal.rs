use bevy::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};
use bevy_rapier2d::prelude::*;

use crate::config;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CrystalColor {
    // Primary
    Blue,
    Red,
    Yellow,

    // Secondary
    Green,
    Purple,
    Orange,

    // Tertiary
    BlueGreen,
    BluePurple,
    RedPurple,
    RedOrange,
    YellowOrange,
    YellowGreen,
}

impl CrystalColor {
    pub fn random_primary() -> Self {
        use CrystalColor::*;
        match rand::random::<u8>() % 3 {
            0 => Blue,
            1 => Red,
            2 => Yellow,
            _ => unreachable!(),
        }
    }

    pub fn to_color(&self) -> Color {
        match self {
            CrystalColor::Blue => Color::rgb(0.0, 0.0, 1.0),
            CrystalColor::Red => Color::rgb(1.0, 0.0, 0.0),
            CrystalColor::Yellow => Color::rgb(1.0, 1.0, 0.0),
            CrystalColor::Green => Color::rgb(0.0, 1.0, 0.0),
            CrystalColor::Purple => Color::rgb(1.0, 0.0, 1.0),
            CrystalColor::Orange => Color::rgb(1.0, 0.5, 0.0),
            CrystalColor::BlueGreen => Color::rgb(0.0, 1.0, 1.0),
            CrystalColor::BluePurple => Color::rgb(0.5, 0.0, 1.0),
            CrystalColor::RedPurple => Color::rgb(1.0, 0.0, 0.5),
            CrystalColor::RedOrange => Color::rgb(1.0, 0.5, 0.0),
            CrystalColor::YellowOrange => Color::rgb(1.0, 1.0, 0.5),
            CrystalColor::YellowGreen => Color::rgb(1.0, 1.0, 0.0),
        }
    }

    pub fn to_draw_mode(&self) -> DrawMode {
        DrawMode::Outlined {
            fill_mode: bevy_prototype_lyon::prelude::FillMode::color(self.to_color()),
            outline_mode: StrokeMode::new(Color::WHITE, config::GRID_WIDTH),
        }
    }

    pub fn is_primary(&self) -> bool {
        match self {
            CrystalColor::Red | CrystalColor::Blue | CrystalColor::Yellow => true,
            _ => false,
        }
    }

    pub fn is_secondary(&self) -> bool {
        match self {
            CrystalColor::Purple | CrystalColor::Green | CrystalColor::Orange => true,
            _ => false,
        }
    }

    pub fn is_tertiary(&self) -> bool {
        !self.is_primary() && !self.is_secondary()
    }

    pub fn combine(&self, other: &Self) -> Option<Self> {
        match (self, other) {
            (CrystalColor::Blue, CrystalColor::Red) => Some(CrystalColor::Purple),
            (CrystalColor::Red, CrystalColor::Blue) => Some(CrystalColor::Purple),
            (CrystalColor::Blue, CrystalColor::Yellow) => Some(CrystalColor::Green),
            (CrystalColor::Yellow, CrystalColor::Blue) => Some(CrystalColor::Green),
            (CrystalColor::Red, CrystalColor::Yellow) => Some(CrystalColor::Orange),
            (CrystalColor::Yellow, CrystalColor::Red) => Some(CrystalColor::Orange),
            (CrystalColor::Orange, CrystalColor::Red) => Some(CrystalColor::RedOrange),
            (CrystalColor::Red, CrystalColor::Orange) => Some(CrystalColor::RedOrange),
            (CrystalColor::Orange, CrystalColor::Yellow) => Some(CrystalColor::YellowOrange),
            (CrystalColor::Yellow, CrystalColor::Orange) => Some(CrystalColor::YellowOrange),
            (CrystalColor::Green, CrystalColor::Blue) => Some(CrystalColor::BlueGreen),
            (CrystalColor::Blue, CrystalColor::Green) => Some(CrystalColor::BlueGreen),
            (CrystalColor::Green, CrystalColor::Yellow) => Some(CrystalColor::YellowGreen),
            (CrystalColor::Yellow, CrystalColor::Green) => Some(CrystalColor::YellowGreen),
            (CrystalColor::Purple, CrystalColor::Blue) => Some(CrystalColor::BluePurple),
            (CrystalColor::Blue, CrystalColor::Purple) => Some(CrystalColor::BluePurple),
            (CrystalColor::Purple, CrystalColor::Red) => Some(CrystalColor::RedPurple),
            (CrystalColor::Red, CrystalColor::Purple) => Some(CrystalColor::RedPurple),
            _ => None,
        }
    }
}

#[derive(Component)]
pub struct Crystal {
    pub crystal_color: CrystalColor,
}

#[derive(Bundle)]
pub struct CrystalBundle {
    #[bundle]
    pub shape_bundle: ShapeBundle,
    pub crystal: Crystal,
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub collider: Collider,
    pub gravity: GravityScale,
    pub locked_axes: LockedAxes,
}

impl CrystalBundle {
    pub fn random_primary() -> Self {
        let crystal_color = CrystalColor::random_primary();
        Self::default().with_color(crystal_color)
    }
    pub fn with_color(mut self, crystal_color: CrystalColor) -> Self {
        self.shape_bundle.mode = crystal_color.to_draw_mode();
        self
    }

    pub fn with_position(mut self, x: f32, y: f32) -> Self {
        self.shape_bundle.transform = Transform::from_translation(Vec3::new(x, y, 100.));
        self
    }
}

impl Default for CrystalBundle {
    fn default() -> Self {
        let color = CrystalColor::Blue;
        let shape = shapes::RegularPolygon {
            sides: 6,
            feature: shapes::RegularPolygonFeature::Radius(1.),
            ..shapes::RegularPolygon::default()
        };
        Self {
            crystal: Crystal {
                crystal_color: color,
            },
            shape_bundle: GeometryBuilder::build_as(
                &shape,
                color.to_draw_mode(),
                Transform::from_translation(Vec3::new(0., 0., 100.)),
            ),
            rigid_body: RigidBody::Dynamic,
            velocity: Velocity::default(),
            collider: Collider::ball(1.),
            gravity: GravityScale(0.),
            locked_axes: LockedAxes::TRANSLATION_LOCKED,
        }
    }
}
