use crate::{
    config::{AppState, GameState},
    core::{
        crystal::{Crystal, CrystalBundle, CrystalColor},
        input,
        platform::PlatformBundle,
        player::{Player, PlayerBundle},
    },
};
use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_prototype_lyon::prelude::{DrawMode, StrokeMode};
use bevy_rapier2d::prelude::*;
use std::f32::consts::SQRT_2;

pub struct MapConfig {
    pub map_size: Vec2,
    pub grid_width: f32,
    pub grid_color: Color,
    pub crystal_linvel: f32,
    pub crystal_angvel: f32,
    pub player_default_color: Color,
    pub player_outline_color: Color,
}

const MAP_CONFIG: MapConfig = MapConfig {
    map_size: Vec2::new(100., 32.),
    grid_width: 0.05,
    grid_color: Color::rgb(0.5, 0.5, 0.5),
    crystal_linvel: 6.,
    crystal_angvel: 4.,
    player_default_color: Color::WHITE,
    player_outline_color: Color::rgb(0., 0.5, 0.2),
};
const HMAP_SIZE: Vec2 = Vec2::new(MAP_CONFIG.map_size.x / 2., MAP_CONFIG.map_size.y / 2.);

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
                .with_system(camera_follow)
                .with_system(crystal_treadmill)
                .with_system(crystal_collision)
                .with_system(colorizer),
        );
    }
}

fn startup(mut commands: Commands) {
    let size = MAP_CONFIG.map_size;
    let hsize = size / 2.;

    // Horizontal lines
    for i in 0..=MAP_CONFIG.map_size.y as i32 {
        commands.spawn_bundle(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(0., i as f32 - hsize.y, 10.)),
            sprite: Sprite {
                color: Color::rgb(0.5, 0.5, 0.5),
                custom_size: Some(Vec2::new(size.x, MAP_CONFIG.grid_width)),
                ..default()
            },
            ..default()
        });
    }

    // Vertical lines
    for i in 0..=MAP_CONFIG.map_size.x as i32 {
        commands.spawn_bundle(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(i as f32 - hsize.x, 0., 10.)),
            sprite: Sprite {
                color: Color::rgb(0.5, 0.5, 0.5),
                custom_size: Some(Vec2::new(MAP_CONFIG.grid_width, size.y)),
                ..default()
            },
            ..default()
        });
    }

    // Create a player
    commands
        .spawn_bundle(
            PlayerBundle::from_shape(4, 1.5 * SQRT_2)
                .with_color(
                    MAP_CONFIG.player_default_color,
                    MAP_CONFIG.player_outline_color,
                )
                .with_position(-0.5, 0.5)
                .with_gravity(0.5),
        )
        .insert(LockedAxes::ROTATION_LOCKED | LockedAxes::TRANSLATION_LOCKED_X);

    // Spawn a containment cell
    let border_color = Color::rgb(0.5, 0.5, 0.5);
    commands.spawn_bundle(
        PlatformBundle::default()
            .with_color(border_color)
            .with_position(0., -hsize.y)
            .with_size(size.x + 1., 1.),
    );
    commands.spawn_bundle(
        PlatformBundle::default()
            .with_color(border_color)
            .with_position(0., hsize.y)
            .with_size(size.x + 1., 1.),
    );

    // Spawn crystals to collect
    for _ in 0..32 {
        let x = rand::random::<f32>() * size.x - hsize.x;
        let y = rand::random::<f32>() * size.y - hsize.y;
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
        if cur_pos.x.abs() > HMAP_SIZE.x || cur_pos.y.abs() > HMAP_SIZE.y {
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

fn crystal_treadmill(
    mut crystal_query: Query<(&mut Transform, &mut Velocity, &mut Crystal, &mut DrawMode)>,
) {
    let map_size = MAP_CONFIG.map_size;
    let hmap_size = map_size / 2.;

    for (mut transform, mut velocity, mut crystal, mut draw_mode) in &mut crystal_query {
        let cur_pos = transform.translation.xy();

        // Crystals have a constant left to right velocity.
        // Crystals have a constant angular velocity so they look cool :)
        velocity.linvel = Vec2::new(-MAP_CONFIG.crystal_linvel, 0.);
        velocity.angvel = MAP_CONFIG.crystal_angvel; // Dancing and twirling... Dancing and twirling...

        // When they leave the playfield, they are moved to the other side
        // and their color is randomized again.
        if cur_pos.x < -hmap_size.x || crystal.collected {
            transform.translation.x = hmap_size.x;
            // Further offset the position randomly to avoid patterns
            transform.translation.x += rand::random::<f32>() * 4.;
            transform.translation.y = rand::random::<f32>() * map_size.y - hmap_size.y;

            transform.rotation = Quat::IDENTITY;

            crystal.crystal_color = CrystalColor::random_primary();
            *draw_mode = crystal.crystal_color.to_draw_mode();

            crystal.collected = false;
        }
    }
}

/// Collision detection between player and crystals.
/// When a player collides with a crystal, the crystal is destroyed and the player's color
/// is changed toward the crystal's color (or to it if the player is empty).
/// The Crystal is respawned at the right edge of the map with a random color.
fn crystal_collision(
    mut collision_events: EventReader<CollisionEvent>,
    mut players: Query<&mut Player>,
    mut crystals: Query<&mut Crystal>,
) {
    for event in collision_events.iter() {
        if let CollisionEvent::Started(entity_a, entity_b, _) = &event {
            // Determine which entity is the player and which is the crystal
            let (player_entity, crystal_entity) = if players.get(*entity_a).is_ok() {
                (*entity_a, *entity_b)
            } else if players.get(*entity_b).is_ok() {
                (*entity_b, *entity_a)
            } else {
                continue;
            };
            if let Ok(mut player) = players.get_mut(player_entity) {
                if let Ok(mut crystal) = crystals.get_mut(crystal_entity) {
                    // Player and crystal are touching, change the player's color
                    match player.color {
                        Some(color) => {
                            player.color = Some(color.combine(&crystal.crystal_color));
                        }
                        None => {
                            player.color = Some(crystal.crystal_color);
                        }
                    }

                    // Don't actually despawn, just mark as collected and let the treadmill handle it
                    crystal.collected = true;
                }
            }
        }
    }
}

/// Loops through the player/crystals and sets the draw mode to match the corresponding color
fn colorizer(
    mut player_query: Query<(&mut DrawMode, &Player, Without<Crystal>)>,
    mut crystal_query: Query<(&mut DrawMode, &Crystal, Without<Player>)>,
) {
    for (mut draw_mode, player, _) in &mut player_query {
        let color = match player.color {
            Some(crystal_color) => crystal_color.to_color(),
            None => MAP_CONFIG.player_default_color,
        };

        *draw_mode = DrawMode::Outlined {
            fill_mode: bevy_prototype_lyon::prelude::FillMode::color(color),
            outline_mode: StrokeMode::new(
                MAP_CONFIG.player_outline_color,
                crate::core::player::PLAYER_OUTLINE_WIDTH,
            ),
        }
    }

    for (mut draw_mode, crystal, _) in &mut crystal_query {
        *draw_mode = crystal.crystal_color.to_draw_mode();
    }
}
