use bevy::prelude::*;
//use bevy_ggrs::*;

pub struct LocalPlayerHandle(pub usize);

#[derive(Component)]
pub struct Player {
    pub handle: usize,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    #[bundle]
    pub sprite_bundle: SpriteBundle,
    pub player: Player,
    pub rollback: bevy_ggrs::Rollback,
}

impl PlayerBundle {
    pub fn new(
        handle: usize,
        color: Color,
        position: Vec3,
        rollback_id: u32,
    ) -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                transform: Transform::from_translation(position),
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(1., 1.)),
                    ..default()
                },
                ..default()
            },
            player: Player { handle },
            rollback: bevy_ggrs::Rollback::new(rollback_id),
        }
    }
}
