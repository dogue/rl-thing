use bevy::prelude::*;
use bevy_pancam::PanCamPlugin;

mod player;
mod tween;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PanCamPlugin::default())
        .add_plugin(player::PlayerPlugin)
        .add_plugin(tween::TweenPlugin)
        .add_system(bevy::window::close_on_esc)
        .run();
}
