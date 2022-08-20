pub const MAP_SIZE: u32 = 41;
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

impl GameConfig {
    pub fn default() -> Self {
        GameConfig::new(
            "PrismPals",
            "prismpals",
            "ws://localhost:3536",
            "wss://matchbox.tracteur.dev:2083",
        )
    }

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
pub enum GameState {
    Menu,
    Coop,
}
