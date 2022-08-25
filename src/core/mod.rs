use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy_ggrs::*;
use ggrs::Config;

use crate::{
    config::{AppState, GameState},
    core::player::Player,
};

pub mod component;
pub mod input;
pub mod player;

pub fn build(app: &mut App) {
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
        let limit = Vec2::splat(crate::config::MAP_SIZE as f32 / 2. - 0.5);
        let new_pos = (old_pos + move_delta).clamp(-limit, limit);

        transform.translation.x = new_pos.x;
        transform.translation.y = new_pos.y;
    }
}
