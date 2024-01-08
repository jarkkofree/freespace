use bevy::prelude::*;

pub struct PlanetPlugin;

impl Plugin for PlanetPlugin {
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

    com.insert_resource(ClearColor(config.clear_color));

    let mut mesh = Mesh::try_from(shape::Icosphere {
        radius: 100.0,
        subdivisions: 20,
    }).unwrap();

    mesh.duplicate_vertices();
    mesh.compute_flat_normals();

    let material = StandardMaterial {
        base_color: Color::YELLOW_GREEN,
        ..default()
    };

    com.spawn(
        PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(material),
            transform: Transform::IDENTITY,
            ..default()
        }
    );

    // spawn point light
    com.spawn(PointLightBundle {
        transform: Transform::from_xyz(0.0, 110.0, 0.0),
        point_light: PointLight {
            color: Color::WHITE,
            intensity: 1_000.0,
            range: 100.0,
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });

    com.insert_resource(config);
}