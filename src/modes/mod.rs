use bevy::prelude::*;

mod coop;
pub mod gym;

pub fn build(app: &mut App) {
    gym::build(app);
    coop::build(app);
}
