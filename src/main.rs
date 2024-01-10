use bevy::prelude::*;

mod player;
use player::PlayerPlugin;

mod planet;
use planet::PlanetPlugin;

mod star;
use star::StarPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PlayerPlugin,
            PlanetPlugin,
        ))
        .run();
}