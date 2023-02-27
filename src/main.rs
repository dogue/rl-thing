use bevy::prelude::*;

pub mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(player::PlayerPlugin)
        .add_system(bevy::window::close_on_esc)
        .run();
}
