use bevy::{math::Vec3Swizzles, prelude::*, render::camera::ScalingMode, tasks::IoTaskPool};
use bevy_ggrs::*;
use ggrs::{Config, PlayerType};
use matchbox_socket::WebRtcSocket;

use crate::{
    config::{GRID_WIDTH, MAP_SIZE},
    input::*,
    GameConfig, GameState, LocalPlayerHandle, Player,
};

pub fn build(app: &mut App) {
    GGRSPlugin::<GgrsConfig>::new()
        .with_input_system(input)
        .with_rollback_schedule(
            Schedule::default().with_stage(
                "ROLLBACK_STAGE",
                SystemStage::single_threaded().with_system_set(
                    SystemSet::on_update(GameState::Coop).with_system(move_players),
                ),
            ),
        )
        .register_rollback_type::<Transform>()
        .build(app);

    app.add_system_set(
        SystemSet::on_enter(GameState::Coop)
            .with_system(setup_board)
            .with_system(start_matchbox_socket)
            .with_system(spawn_players),
    )
    .add_system_set(
        SystemSet::on_update(GameState::Coop)
            .with_system(wait_for_players)
            .with_system(camera_follow),
    );
}

#[derive(Debug)]
struct GgrsConfig;
impl Config for GgrsConfig {
    // 4-directions + fire fits easily in a single byte
    type Input = u8;
    type State = u8;
    // Matchbox' WebRtcSocket addresses are strings
    type Address = String;
}

fn setup_board(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 1.,
            scaling_mode: ScalingMode::FixedVertical(24.0),
            ..default()
        },
        ..default()
    });

    // Horizontal lines
    for i in 0..=MAP_SIZE {
        commands.spawn_bundle(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(
                0.,
                i as f32 - MAP_SIZE as f32 / 2.,
                10.,
            )),
            sprite: Sprite {
                color: Color::rgb(0.27, 0.27, 0.27),
                custom_size: Some(Vec2::new(MAP_SIZE as f32, GRID_WIDTH)),
                ..default()
            },
            ..default()
        });
    }

    // Vertical lines
    for i in 0..=MAP_SIZE {
        commands.spawn_bundle(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(
                i as f32 - MAP_SIZE as f32 / 2.,
                0.,
                10.,
            )),
            sprite: Sprite {
                color: Color::rgb(0.27, 0.27, 0.27),
                custom_size: Some(Vec2::new(GRID_WIDTH, MAP_SIZE as f32)),
                ..default()
            },
            ..default()
        });
    }
}

fn start_matchbox_socket(mut commands: Commands, game_config: Res<GameConfig>) {
    // If the MATCHBOX_SERVER environment variable is set, use it.
    // Otherwise, in debug mode, use localhost.
    //   and in release mode, use the production matchbox server.
    let host = match std::env::var("MATCHBOX_SERVER") {
        Ok(url) => url,
        Err(_) => {
            if cfg!(debug_assertions) {
                game_config.debug_matchbox_server.to_string()
            } else {
                game_config.release_matchbox_server.to_string()
            }
        }
    };

    let room_url = format!(
        "{}/{}_{}.{}?next=2",
        host,
        game_config.game_slug,
        game_config.build_version.major,
        game_config.build_version.minor
    );

    info!("connecting to matchbox server: {:?}", room_url);
    let (socket, message_loop) = WebRtcSocket::new(room_url);

    // The message loop needs to be awaited, or nothing will happen.
    // We do this here using bevy's task system.
    let task_pool = IoTaskPool::get();
    task_pool.spawn(message_loop).detach();

    commands.insert_resource(Some(socket));
}

fn spawn_players(mut commands: Commands, mut rip: ResMut<RollbackIdProvider>) {
    // Player 1
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(-2., 0., 100.)),
            sprite: Sprite {
                color: Color::rgb(0., 0.47, 1.),
                custom_size: Some(Vec2::new(1., 1.)),
                ..default()
            },
            ..default()
        })
        .insert(Player { handle: 0 })
        .insert(Rollback::new(rip.next_id()));

    // Player 2
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(2., 0., 100.)),
            sprite: Sprite {
                color: Color::rgb(0., 0.4, 0.),
                custom_size: Some(Vec2::new(1., 1.)),
                ..default()
            },
            ..default()
        })
        .insert(Player { handle: 1 })
        .insert(Rollback::new(rip.next_id()));
}

fn wait_for_players(mut commands: Commands, mut socket: ResMut<Option<WebRtcSocket>>) {
    let socket = socket.as_mut();

    // If there is no socket we've already started the game
    if socket.is_none() {
        return;
    }

    // Check for new connections
    socket.as_mut().unwrap().accept_new_connections();
    let players = socket.as_ref().unwrap().players();

    let num_players = 2;
    if players.len() < num_players {
        return; // wait for more players
    }

    info!("All peers have joined, going in-game");

    // create a GGRS P2P session
    let mut session_builder = ggrs::SessionBuilder::<GgrsConfig>::new()
        .with_num_players(num_players)
        .with_input_delay(2);

    for (i, player) in players.into_iter().enumerate() {
        if player == PlayerType::Local {
            commands.insert_resource(LocalPlayerHandle(i));
        }

        session_builder = session_builder
            .add_player(player, i)
            .expect("failed to add player");
    }

    // move the socket out of the resource (required because GGRS takes ownership of it)
    let socket = socket.take().unwrap();

    // start the GGRS session
    let session = session_builder
        .start_p2p_session(socket)
        .expect("failed to start session");

    commands.insert_resource(session);
    commands.insert_resource(SessionType::P2PSession);
}

fn camera_follow(
    player_handle: Option<Res<LocalPlayerHandle>>,
    player_query: Query<(&Transform, &Player)>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    let player_handle = match player_handle {
        Some(handle) => handle.0,
        None => return, // Session hasn't started yet
    };

    for (player_transform, player) in &player_query {
        if player.handle != player_handle {
            continue;
        }

        let pos = player_transform.translation;

        for mut transform in &mut camera_query {
            transform.translation.x = pos.x;
            transform.translation.y = pos.y;
        }
    }
}

fn move_players(
    inputs: Res<Vec<(u8, ggrs::InputStatus)>>,
    mut player_query: Query<(&mut Transform, &Player)>,
) {
    for (mut transform, player) in &mut player_query {
        let (input, _) = inputs[player.handle];
        let direction = direction(input);

        if direction == Vec2::ZERO {
            continue;
        }

        let move_speed = 0.13;
        let move_delta = direction * move_speed;

        let old_pos = transform.translation.xy();
        let limit = Vec2::splat(MAP_SIZE as f32 / 2. - 0.5);
        let new_pos = (old_pos + move_delta).clamp(-limit, limit);

        transform.translation.x = new_pos.x;
        transform.translation.y = new_pos.y;
    }
}
