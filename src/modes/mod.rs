use bevy::prelude::*;

mod bump;

pub struct ModesPlugin;

impl Plugin for ModesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(bump::BumpPlugin);
    }
}
