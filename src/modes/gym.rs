use crate::{
    config::{self, AppState, GameState},
    core::{
        input,
        player::{Player, PlayerBundle},
        platform::PlatformBundle,
    },
};
use bevy::{math::Vec3Swizzles, prelude::*};

pub fn build(app: &mut App) {
    app.add_system_set(SystemSet::on_enter(AppState::Game(GameState::Gym)).with_system(startup));
    // app.add_system_set(SystemSet::on_exit(AppState::Game(GameState::Gym)).with_system(shutdown));
    app.add_system_set(
        SystemSet::on_update(AppState::Game(GameState::Gym))
            .with_system(move_player)
            .with_system(camera_follow),
    );
}

fn startup(mut commands: Commands) {
    // Horizontal lines
    for i in 0..=config::MAP_SIZE {
        commands.spawn_bundle(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(
                0.,
                i as f32 - config::MAP_SIZE as f32 / 2.,
                10.,
            )),
            sprite: Sprite {
                color: Color::rgb(0.5, 0.5, 0.5),
                custom_size: Some(Vec2::new(config::MAP_SIZE as f32, config::GRID_WIDTH)),
                ..default()
            },
            ..default()
        });
    }

    // Vertical lines
    for i in 0..=config::MAP_SIZE {
        commands.spawn_bundle(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(
                i as f32 - config::MAP_SIZE as f32 / 2.,
                0.,
                10.,
            )),
            sprite: Sprite {
                color: Color::rgb(0.5, 0.5, 0.5),
                custom_size: Some(Vec2::new(config::GRID_WIDTH, config::MAP_SIZE as f32)),
                ..default()
            },
            ..default()
        });
    }

    // Create a player
    commands.spawn_bundle(PlayerBundle::new(
        0,
        Color::rgb(0.4, 0.0, 0.6),
        Vec3::new(-0.5, 0.5, 100.),
        0,
    ));

    // Spawn a platform
    commands.spawn_bundle(PlatformBundle::new(
        Color::rgb(0.6, 0.6, 0.2),
        Vec3::new(0., -1.5, 50.),
        Vec2::new(16., 1.),
    ));
}

// fn shutdown(mut commands: Commands) {}

fn move_player(keys: Res<Input<KeyCode>>, mut player_query: Query<(&mut Transform, &Player)>) {
    let direction = input::direction(input::input(keys));
    for (mut transform, _) in &mut player_query {
        if direction == Vec2::ZERO {
            continue;
        }

        let move_speed = 0.07;
        let move_delta = direction.normalize_or_zero() * move_speed;

        let old_pos = transform.translation.xy();
        let limit = Vec2::splat(crate::config::MAP_SIZE as f32 / 2. - 0.5);
        let new_pos = (old_pos + move_delta).clamp(-limit, limit);

        transform.translation.x = new_pos.x;
        transform.translation.y = new_pos.y;
    }
}

fn camera_follow(
    player_query: Query<(&Transform, &Player)>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    for (player_transform, player) in &player_query {
        if player.handle != 0 {
            continue;
        }

        let pos = player_transform.translation;

        for mut transform in &mut camera_query {
            transform.translation.x = pos.x;
            transform.translation.y = pos.y;
        }
    }
}
