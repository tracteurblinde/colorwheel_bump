use bevy::prelude::Color;
pub const MAP_SIZE: u32 = 42;
pub const GRID_WIDTH: f32 = 0.05;

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct GameConfig {
    pub game_title: &'static str,
    pub game_slug: &'static str,
    pub debug_matchbox_server: &'static str,
    pub release_matchbox_server: &'static str,
    pub build_timestamp: &'static str,
    pub build_version: semver::Version,
}

impl Default for GameConfig {
    fn default() -> Self {
        GameConfig::new(
            "PrismPals",
            "prismpals",
            "ws://localhost:3536",
            "wss://matchbox.tracteur.dev:2083",
        )
    }
}

impl GameConfig {
    pub fn new(
        game_title: &'static str,
        game_slug: &'static str,
        debug_matchbox_server: &'static str,
        release_matchbox_server: &'static str,
    ) -> Self {
        let build_version = semver::Version::parse(
            format!(
                "{}+{}",
                env!("VERGEN_BUILD_SEMVER"),
                env!("VERGEN_GIT_SHA_SHORT")
            )
            .as_str(),
        )
        .unwrap();
        let build_timestamp = env!("VERGEN_BUILD_TIMESTAMP");

        GameConfig {
            game_title,
            game_slug,
            debug_matchbox_server,
            release_matchbox_server,
            build_timestamp,
            build_version,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MenuState {
    Main,
    Credits,
}

#[derive(Debug, Clone, Hash)]
pub enum GameState {
    Gym,
    Coop,
    Any,
}

impl PartialEq for GameState {
    fn eq(&self, other: &Self) -> bool {
        // Return true if either side is Any.
        // Otherwise, do a normal comparison.
        match (self, other) {
            (GameState::Any, _) => true,
            (_, GameState::Any) => true,
            (GameState::Gym, GameState::Gym) => true,
            (GameState::Coop, GameState::Coop) => true,
            _ => false,
        }
    }
}
impl Eq for GameState {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    Menu(MenuState),
    Game(GameState),
    GameGym,
}

pub const BUTTON_COLOR: Color = Color::rgb(0.27, 0.27, 0.27);
pub const BUTTON_HOVER_COLOR: Color = Color::rgb(0.2, 0.2, 0.2);
pub const BUTTON_PRESSED_COLOR: Color = Color::rgb(0.3, 0.3, 0.3);