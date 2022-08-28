use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

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
    pub sprite_bundle: SpriteBundle,
    pub crystal: Crystal,
    pub collider: Collider,
    pub rigid_body: RigidBody,
}

impl CrystalBundle {
    pub fn random_primary() -> Self {
        let crystal_color = CrystalColor::random_primary();
        Self::default().with_color(crystal_color)
    }
    pub fn with_color(mut self, crystal_color: CrystalColor) -> Self {
        self.crystal.crystal_color = crystal_color;
        self.sprite_bundle.sprite.color = crystal_color.to_color();
        self
    }

    pub fn with_position(mut self, x: f32, y: f32) -> Self {
        self.sprite_bundle.transform = Transform::from_translation(Vec3::new(x, y, 75.));
        self
    }

    pub fn with_size(mut self, size: Vec2) -> Self {
        self.sprite_bundle.sprite.custom_size = Some(size);
        self.collider = Collider::cuboid(size.x / 2., size.y / 2.);
        self
    }
}

impl Default for CrystalBundle {
    fn default() -> Self {
        let color = CrystalColor::Blue;
        Self {
            crystal: Crystal {
                crystal_color: color,
            },
            sprite_bundle: SpriteBundle {
                transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
                sprite: Sprite {
                    color: color.to_color(),
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
