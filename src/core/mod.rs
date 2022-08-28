use bevy::{math::Vec3Swizzles, prelude::*, render::camera::ScalingMode};
use bevy_ggrs::*;
use bevy_prototype_lyon::prelude::ShapePlugin;
use bevy_rapier2d::prelude::*;
use ggrs::Config;

use crate::{
    config::{AppState, GameState},
    core::player::Player,
};

pub mod component;
pub mod crystal;
pub mod input;
pub mod platform;
pub mod player;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        GGRSPlugin::<GgrsConfig>::new()
            .with_input_system(input::input_mp)
            .with_rollback_schedule(Schedule::default().with_stage(
                "ROLLBACK_STAGE",
                SystemStage::single_threaded().with_system_set(
                    SystemSet::on_update(AppState::Game(GameState::Any)).with_system(move_players),
                ),
            ))
            .register_rollback_type::<Transform>()
            .build(app);

        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .insert_resource(Msaa { samples: 4 })
            .add_plugin(ShapePlugin)
            .add_startup_system(spawn_camera);

        if cfg!(debug_assertions) {
            app.add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
                //.add_plugin(RapierDebugRenderPlugin::default())
                ;
        }
    }
}

#[derive(Debug)]
pub struct GgrsConfig;
impl Config for GgrsConfig {
    // 4-directions + fire fits easily in a single byte
    type Input = u8;
    type State = u8;
    // Matchbox' WebRtcSocket addresses are strings
    type Address = String;
}

fn move_players(
    inputs: Res<Vec<(u8, ggrs::InputStatus)>>,
    mut player_query: Query<(&mut Transform, &Player)>,
) {
    for (mut transform, player) in &mut player_query {
        let (input, _) = inputs[player.handle];
        let direction = input::direction(input);

        if direction == Vec2::ZERO {
            continue;
        }

        let move_speed = 0.13;
        let move_delta = direction * move_speed;

        let old_pos = transform.translation.xy();
        let new_pos = old_pos + move_delta;

        transform.translation.x = new_pos.x;
        transform.translation.y = new_pos.y;
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 1.,
            scaling_mode: ScalingMode::FixedVertical(21.),
            ..default()
        },
        ..default()
    });
}
