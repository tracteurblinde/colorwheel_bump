use bevy::prelude::*;

mod gym;

pub struct ModesPlugin;

impl Plugin for ModesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(gym::GymPlugin);
    }
}
