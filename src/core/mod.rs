use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_prototype_lyon::prelude::ShapePlugin;
use bevy_rapier2d::prelude::*;

pub mod component;
pub mod crystal;
pub mod platform;
pub mod player;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
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
