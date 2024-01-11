use bevy::prelude::*;

mod player;
use player::PlayerPlugin;

mod planet;
use planet::PlanetPlugin;

mod star_names;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PlayerPlugin,
            PlanetPlugin,
        ))
        .run();
}