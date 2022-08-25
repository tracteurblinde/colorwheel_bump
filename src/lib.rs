use bevy::{prelude::*, render::camera::ScalingMode};

use config::*;
pub mod config;

mod core;
mod menu;
mod modes;

pub fn app() -> App {
    let game_config = GameConfig::default();
    let mut app = App::new();

    app.insert_resource(game_config.clone())
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .insert_resource(WindowDescriptor {
            title: game_config.game_title.to_string(),
            canvas: Some("#bevy".to_string()),
            fit_canvas_to_parent: true,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera)
        .add_state(AppState::Game(GameState::Gym));

    core::build(&mut app);
    menu::build(&mut app);
    modes::build(&mut app);

    app
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 1.,
            scaling_mode: ScalingMode::FixedVertical(24.0),
            ..default()
        },
        ..default()
    });
}
