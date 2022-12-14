use bevy::prelude::*;
use crate::config::*;
use crate::core::CorePlugin;
use crate::bump::BumpPlugin;

pub mod config;
mod core;
mod bump;

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
        .add_plugin(CorePlugin)
        .add_plugin(BumpPlugin);

    app
}
