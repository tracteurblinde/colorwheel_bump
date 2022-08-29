use bevy::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};
use bevy_rapier2d::prelude::*;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter, FromPrimitive)]
pub enum CrystalColor {
    Orange = 0,
    YellowOrange,
    Yellow,
    YellowGreen,
    Green,
    BlueGreen,
    Blue,
    BluePurple,
    Purple,
    RedPurple,
    Red,
    RedOrange,
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

    #[allow(dead_code)]
    pub fn random_secondary() -> Self {
        use CrystalColor::*;
        match rand::random::<u8>() % 3 {
            0 => Orange,
            1 => Green,
            2 => Purple,
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub fn random_tertiary() -> Self {
        use CrystalColor::*;
        match rand::random::<u8>() % 6 {
            0 => YellowOrange,
            1 => YellowGreen,
            2 => BlueGreen,
            3 => BluePurple,
            4 => RedPurple,
            5 => RedOrange,
            _ => unreachable!(),
        }
    }

    pub fn to_color(&self) -> Color {
        match self {
            CrystalColor::Orange => Color::rgb_u8(255, 126, 0),
            CrystalColor::YellowOrange => Color::rgb_u8(255, 218, 48),
            CrystalColor::Yellow => Color::rgb_u8(255, 255, 0),
            CrystalColor::YellowGreen => Color::rgb_u8(192, 233, 17),
            CrystalColor::Green => Color::rgb_u8(38, 155, 38),
            CrystalColor::BlueGreen => Color::rgb_u8(0, 141, 136),
            CrystalColor::Blue => Color::rgb_u8(19, 49, 192),
            CrystalColor::BluePurple => Color::rgb_u8(109, 83, 192),
            CrystalColor::Purple => Color::rgb_u8(114, 51, 143),
            CrystalColor::RedPurple => Color::rgb_u8(183, 47, 165),
            CrystalColor::Red => Color::rgb_u8(239, 1, 1),
            CrystalColor::RedOrange => Color::rgb_u8(255, 62, 0),
        }
    }

    pub fn to_draw_mode(&self) -> DrawMode {
        DrawMode::Outlined {
            fill_mode: bevy_prototype_lyon::prelude::FillMode::color(self.to_color()),
            outline_mode: StrokeMode::new(Color::WHITE, 0.05),
        }
    }

    #[allow(dead_code)]
    pub fn is_primary(&self) -> bool {
        match self {
            CrystalColor::Red | CrystalColor::Blue | CrystalColor::Yellow => true,
            _ => false,
        }
    }

    #[allow(dead_code)]
    pub fn is_secondary(&self) -> bool {
        match self {
            CrystalColor::Purple | CrystalColor::Green | CrystalColor::Orange => true,
            _ => false,
        }
    }

    #[allow(dead_code)]
    pub fn is_tertiary(&self) -> bool {
        !self.is_primary() && !self.is_secondary()
    }

    pub fn combine(&self, other: &Self) -> Self {
        let old_color: i8 = *self as i8;
        let new_color: i8 = *other as i8;
        let num_colors = CrystalColor::iter().count() as i8;
        let half_num_colors = (CrystalColor::iter().count() / 2) as i8;

        // TODO: I'm very confident there's a way to do this without branching
        let new_color = if new_color > old_color {
            if new_color - old_color <= half_num_colors {
                old_color + 1
            } else {
                old_color - 1
            }
        } else if new_color < old_color {
            if old_color - new_color <= half_num_colors {
                old_color - 1
            } else {
                old_color + 1
            }
        } else {
            old_color
        };

        CrystalColor::from_i8((new_color + num_colors) % num_colors).unwrap()
    }
}

#[derive(Component)]
pub struct Crystal {
    pub crystal_color: CrystalColor,
    pub collected: bool,
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
        self.crystal.crystal_color = crystal_color;
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
                collected: false,
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
