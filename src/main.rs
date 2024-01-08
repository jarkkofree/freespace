use bevy::prelude::*;

mod player;
use player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PlayerPlugin,
        ))
        .run();
}