use bevy::prelude::*;

mod player;
use player::PlayerPlugin;

mod planet;
use planet::PlanetPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PlayerPlugin,
            PlanetPlugin,
        ))
        .run();
}