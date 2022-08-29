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
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use std::f32::consts::SQRT_2;
use strum::IntoEnumIterator;

pub struct MapConfig {
    pub map_size: Vec2,
    pub grid_width: f32,
    pub grid_color: Color,
    pub crystal_linvel: f32,
    pub crystal_angvel: f32,
    pub player_default_color: Color,
    pub player_outline_color: Color,
    pub background_speed: f32,
    pub colorwheel_height: f32,
    pub colorwheel_radius: f32,
}

const MAP_CONFIG: MapConfig = MapConfig {
    map_size: Vec2::new(100., 32.),
    grid_width: 0.05,
    grid_color: Color::rgb(0.5, 0.5, 0.5),
    crystal_linvel: 6.,
    crystal_angvel: 4.,
    player_default_color: Color::WHITE,
    player_outline_color: Color::WHITE,
    background_speed: 3.,
    colorwheel_height: 8.,
    colorwheel_radius: 2.,
};
const HMAP_SIZE: Vec2 = Vec2::new(MAP_CONFIG.map_size.x / 2., MAP_CONFIG.map_size.y / 2.);

#[derive(Component)]
struct VerticalLine;

#[derive(Component)]
struct ColorWheelWedge {
    crystal_color: CrystalColor,
    is_target: bool,
}

#[derive(Component)]
struct ColorWheelIndicator {}

#[derive(Component)]
struct ColorWheel {}

#[derive(Clone, Debug)]
#[allow(dead_code)]
struct CurrentColor {
    crystal_color: Option<CrystalColor>,
}

pub struct BumpPlugin;

impl Plugin for BumpPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentColor {
            crystal_color: None,
        })
        .add_system_set(
            SystemSet::on_enter(AppState::Game(GameState::Bump))
                .with_system(startup)
                .with_system(startup_colorwheel),
        )
        //.add_system_set(SystemSet::on_exit(AppState::Game(GameState::Bump)).with_system(shutdown))
        .add_system_set(
            SystemSet::on_update(AppState::Game(GameState::Bump))
                .with_system(move_player)
                .with_system(camera_follow)
                .with_system(background_treadmill)
                .with_system(crystal_treadmill)
                .with_system(crystal_collision)
                .with_system(colorizer)
                .with_system(colorwheel_follow)
                .with_system(colorwheel_indicator_update)
                .with_system(colorwheel_wedge_update),
        );
    }
}

fn startup(mut commands: Commands) {
    let size = MAP_CONFIG.map_size;
    let hsize = size / 2.;

    // Horizontal lines
    for i in 0..=MAP_CONFIG.map_size.y as i32 {
        commands.spawn_bundle(SpriteBundle {
            transform: Transform::from_xyz(0., i as f32 - hsize.y, 10.),
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
        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform::from_xyz(i as f32 - hsize.x, 0., 10.),
                sprite: Sprite {
                    color: Color::rgb(0.5, 0.5, 0.5),
                    custom_size: Some(Vec2::new(MAP_CONFIG.grid_width, size.y)),
                    ..default()
                },
                ..default()
            })
            .insert(VerticalLine {});
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
    // TODO: Move the magic constants to a Bump game config
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
    for (player_transform, _) in &player_query {
        let pos = player_transform.translation;

        for mut transform in &mut camera_query {
            transform.translation.x = pos.x;
            transform.translation.y = pos.y;
        }
    }
}

fn background_treadmill(mut background_query: Query<(&mut Transform, &VerticalLine)>) {
    for (mut transform, _) in &mut background_query {
        transform.translation.x -= 0.01;
        if transform.translation.x < -HMAP_SIZE.x {
            transform.translation.x += HMAP_SIZE.x * 2.;
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
    mut current_color: ResMut<CurrentColor>,
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

                    // Update the current color
                    current_color.crystal_color = player.color;
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

fn startup_colorwheel(mut commands: Commands) {
    // Use the shape plugin to draw a color wheel, coloring each of the wheel segments
    //  by iterating through the CrystalColor enum.
    let radius = MAP_CONFIG.colorwheel_radius;
    let height = MAP_CONFIG.colorwheel_height;
    let target_color = CrystalColor::Purple;
    let current_color = CrystalColor::Orange;

    commands
        .spawn()
        .insert(ColorWheel {})
        .insert(Transform::from_xyz(0., height, 90.))
        .insert(GlobalTransform::default())
        .insert(Visibility::default())
        .insert(ComputedVisibility::default())
        .with_children(|parent| {
            parent.spawn_bundle(GeometryBuilder::build_as(
                &shapes::RegularPolygon {
                    sides: CrystalColor::iter().count(),
                    feature: shapes::RegularPolygonFeature::Radius(radius),
                    ..default()
                },
                DrawMode::Outlined {
                    fill_mode: bevy_prototype_lyon::prelude::FillMode::color(Color::rgb(
                        0.1, 0.1, 0.1,
                    )),
                    outline_mode: StrokeMode::new(Color::WHITE, 0.05),
                },
                Transform::from_xyz(0., 0., 0.).with_rotation(Quat::from_rotation_z(
                    std::f32::consts::PI / CrystalColor::iter().count() as f32,
                )),
            ));

            for (i, color) in CrystalColor::iter().enumerate() {
                let angle1 =
                    i as f32 * 2. * std::f32::consts::PI / CrystalColor::iter().count() as f32;

                let angle2 = (i + 1) as f32 * 2. * std::f32::consts::PI
                    / CrystalColor::iter().count() as f32;
                let outside1 = Vec2::new(angle1.cos(), angle1.sin()) * radius;
                let outside2 = Vec2::new(angle2.cos(), angle2.sin()) * radius;
                let inside1 = outside1 * 0.5;
                let inside2 = outside2 * 0.5;

                let shape = shapes::Polygon {
                    points: vec![outside1, outside2, inside2, inside1],
                    closed: true,
                };

                parent
                    .spawn_bundle(GeometryBuilder::build_as(
                        &shape,
                        DrawMode::Outlined {
                            fill_mode: bevy_prototype_lyon::prelude::FillMode::color(
                                color.to_color(),
                            ),
                            outline_mode: StrokeMode::new(Color::WHITE, 0.05),
                        },
                        Transform::from_xyz(0., 0., 1.),
                    ))
                    .insert(ColorWheelWedge {
                        crystal_color: color,
                        is_target: color == target_color,
                    });
            }

            // Draw the current color indicator
            let angle = (current_color as usize as f32 + 0.5) * 2. * std::f32::consts::PI
                / CrystalColor::iter().count() as f32;
            let x = angle.cos() * radius * 0.75;
            let y = angle.sin() * radius * 0.75;
            parent
                .spawn_bundle(GeometryBuilder::build_as(
                    &shapes::RegularPolygon {
                        sides: 4,
                        feature: shapes::RegularPolygonFeature::Radius(0.2),
                        ..default()
                    },
                    DrawMode::Outlined {
                        fill_mode: bevy_prototype_lyon::prelude::FillMode::color(
                            current_color.to_color(),
                        ),
                        outline_mode: StrokeMode::new(Color::WHITE, 0.05),
                    },
                    Transform::from_xyz(x, y, 3.),
                ))
                .insert(ColorWheelIndicator {});
        });
}

fn colorwheel_follow(
    mut colorwheels: Query<(&mut Transform, &ColorWheel, Without<Player>)>,
    players: Query<(&Transform, &Player, Without<ColorWheel>)>,
) {
    for (mut transform, _, _) in colorwheels.iter_mut() {
        for (player_transform, _, _) in players.iter() {
            transform.translation =
                player_transform.translation + Vec3::new(0., MAP_CONFIG.colorwheel_height, 0.);
        }
    }
}

fn colorwheel_wedge_update(
    mut colorwheel_wedges: Query<(&ColorWheelWedge, &mut Transform, &mut DrawMode)>,
    current_color: Res<CurrentColor>,
) {
    let current_color = current_color.crystal_color;
    for (wedge, mut transform, mut draw_mode) in colorwheel_wedges.iter_mut() {
        let mut alpha = 0.1;
        if wedge.crystal_color.is_primary() {
            alpha = 0.4;
        }
        if wedge.is_target {
            alpha = 1.;
        }
        if let Some(color) = current_color {
            if wedge.crystal_color == color {
                alpha = 0.7;
            }
        }

        let mut fill_color = wedge.crystal_color.to_color();
        fill_color.set_a(alpha);

        let border_color = match wedge.is_target {
            true => Color::WHITE,
            false => Color::rgb(0.1, 0.1, 0.1),
        };

        transform.translation = match wedge.is_target {
            true => Vec3::new(0., 0., 2.),
            false => Vec3::new(0., 0., 1.),
        };

        *draw_mode = DrawMode::Outlined {
            fill_mode: bevy_prototype_lyon::prelude::FillMode::color(fill_color),
            outline_mode: StrokeMode::new(border_color, 0.05),
        };
    }
}

fn colorwheel_indicator_update(
    mut colorwheel_indicator: Query<(
        &mut Visibility,
        &mut Transform,
        &mut DrawMode,
        &ColorWheelIndicator,
    )>,
    current_color: Res<CurrentColor>,
) {
    let current_color = current_color.crystal_color;

    if let Some(current_color) = current_color {
        let angle = (current_color as usize as f32 + 0.5) * 2. * std::f32::consts::PI
            / CrystalColor::iter().count() as f32;
        let x = angle.cos() * 2. * 0.75;
        let y = angle.sin() * 2. * 0.75;
        for (mut visibility, mut transform, mut draw_mode, _) in
            colorwheel_indicator.iter_mut()
        {
            transform.translation = Vec3::new(x, y, 3.);
            *draw_mode = DrawMode::Outlined {
                fill_mode: bevy_prototype_lyon::prelude::FillMode::color(current_color.to_color()),
                outline_mode: StrokeMode::new(Color::WHITE, 0.05),
            };
            visibility.is_visible = true;
        }
    } else {
        for (mut visibility, _, _, _) in colorwheel_indicator.iter_mut() {
            visibility.is_visible = false;
        }
    }
}
