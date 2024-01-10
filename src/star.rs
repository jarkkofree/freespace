use bevy::prelude::*;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, startup);
    }
}

#[derive(Resource)]
struct Config {
    clear_color: Color,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            clear_color: Color::rgb(0.1, 0.1, 0.1),
        }
    }
}

fn startup(
    mut com: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let config = Config::default();


    com.insert_resource(config);
}