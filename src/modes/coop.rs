use bevy::{prelude::*, tasks::IoTaskPool};
use bevy_ggrs::*;
use ggrs::PlayerType;
use matchbox_socket::WebRtcSocket;

use crate::{
    config::{GRID_WIDTH, MAP_SIZE},
    core::player::{LocalPlayerHandle, Player, PlayerBundle},
    AppState, GameConfig, GameState,
};

pub struct CoopPlugin;

impl Plugin for CoopPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::Game(GameState::Coop))
                .with_system(setup_board)
                .with_system(start_matchbox_socket)
                .with_system(spawn_players),
        )
        .add_system_set(
            SystemSet::on_update(AppState::Game(GameState::Coop))
                .with_system(wait_for_players)
                .with_system(camera_follow),
        );
    }
}

fn setup_board(mut commands: Commands) {
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
    commands.spawn_bundle(
        PlayerBundle::default()
            .with_id(rip.next_id())
            .with_color(Color::rgb(0., 0.47, 1.))
            .with_position(-2., 0.),
    );

    // Player 2
    commands.spawn_bundle(
        PlayerBundle::default()
            .with_id(rip.next_id())
            .with_color(Color::rgb(0., 0.4, 0.))
            .with_position(-2., 0.),
    );
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
    let mut session_builder = ggrs::SessionBuilder::<crate::core::GgrsConfig>::new()
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
