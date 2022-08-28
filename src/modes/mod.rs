use bevy::prelude::*;

mod coop;
pub mod gym;

pub struct ModesPlugin;

impl Plugin for ModesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(gym::GymPlugin).add_plugin(coop::CoopPlugin);
    }
}
