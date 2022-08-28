use crate::{
    config::{self, AppState, GameState},
    core::{
        crystal::CrystalBundle,
        input,
        platform::PlatformBundle,
        player::{Player, PlayerBundle},
    },
};
use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_rapier2d::prelude::*;

pub struct GymPlugin;

impl Plugin for GymPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::Game(GameState::Gym)).with_system(startup),
        )
        //.add_system_set(SystemSet::on_exit(AppState::Game(GameState::Gym)).with_system(shutdown))
        .add_system_set(
            SystemSet::on_update(AppState::Game(GameState::Gym))
                .with_system(move_player)
                .with_system(camera_follow),
        );
    }
}

fn startup(mut commands: Commands) {
    let map_size = config::MAP_SIZE as f32;

    // Horizontal lines
    for i in 0..=config::MAP_SIZE {
        commands.spawn_bundle(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(0., i as f32 - map_size / 2., 10.)),
            sprite: Sprite {
                color: Color::rgb(0.5, 0.5, 0.5),
                custom_size: Some(Vec2::new(map_size, config::GRID_WIDTH)),
                ..default()
            },
            ..default()
        });
    }

    // Vertical lines
    for i in 0..=config::MAP_SIZE {
        commands.spawn_bundle(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(i as f32 - map_size / 2., 0., 10.)),
            sprite: Sprite {
                color: Color::rgb(0.5, 0.5, 0.5),
                custom_size: Some(Vec2::new(config::GRID_WIDTH, map_size)),
                ..default()
            },
            ..default()
        });
    }

    // Create a player
    commands.spawn_bundle(
        PlayerBundle::default()
            .with_color(Color::rgb(1., 1., 1.))
            .with_size(3., 3.)
            .with_position(-0.5, 0.5)
            .with_gravity(0.5),
    );

    // Spawn a containment cell
    let border_color = Color::rgb(0.5, 0.5, 0.5);
    commands.spawn_bundle(
        PlatformBundle::default()
            .with_color(border_color)
            .with_position(0., -(map_size / 2.))
            .with_size(map_size + 1., 1.),
    );
    commands.spawn_bundle(
        PlatformBundle::default()
            .with_color(border_color)
            .with_position(0., map_size / 2.)
            .with_size(map_size + 1., 1.),
    );
    commands.spawn_bundle(
        PlatformBundle::default()
            .with_color(border_color)
            .with_position(-(map_size / 2.), 0.)
            .with_size(1., map_size + 1.),
    );
    commands.spawn_bundle(
        PlatformBundle::default()
            .with_color(border_color)
            .with_position(map_size / 2., 0.)
            .with_size(1., map_size + 1.),
    );

    // Spawn 10 crystals to collect
    for _ in 0..10 {
        let x = rand::random::<f32>() * map_size - map_size / 2.;
        let y = rand::random::<f32>() * map_size - map_size / 2.;
        commands.spawn_bundle(CrystalBundle::random_primary().with_position(x, y));
    }
}

// fn shutdown(mut commands: Commands) {}

fn move_player(
    keys: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut Velocity, &mut ExternalImpulse, &Player)>,
) {
    // TODO: Move the magic constants to a gym game config
    let move_speed = 0.005;
    let mut move_delta = input::direction(input::input(keys));
    move_delta = move_delta.normalize_or_zero() * move_speed;
    //move_delta.x = 0.0002; // Ever forward, never learning

    for (mut transform, mut velocity, mut external_impulse, _) in &mut player_query {
        external_impulse.impulse = move_delta;
        transform.rotation = Quat::IDENTITY;

        // Clamp the linear velocity
        let max_speed = 15.;
        velocity.linvel = velocity.linvel.clamp(
            Vec2::new(-max_speed, -max_speed),
            Vec2::new(max_speed, max_speed),
        );

        //velocity.linvel = move_delta;

        let cur_pos = transform.translation.xy();
        // If the player is outside the map, move them back to 0,0, clear impulse/velocities
        if cur_pos.x.abs() > config::MAP_SIZE as f32 / 2.
            || cur_pos.y.abs() > config::MAP_SIZE as f32 / 2.
        {
            transform.translation = Vec3::new(0., 0., 100.);
            transform.rotation = Quat::IDENTITY;
            external_impulse.impulse = Vec2::ZERO;
            external_impulse.torque_impulse = 0.;
            velocity.linvel = Vec2::ZERO;
            velocity.angvel = 0.;
        }
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
