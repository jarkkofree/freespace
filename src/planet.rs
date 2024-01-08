use bevy::{prelude::*, pbr::OpaqueRendererMethod};

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

    let planet_radius = 100.0;
    let mut planet_mesh = Mesh::try_from(shape::Icosphere {
        radius: planet_radius,
        subdivisions: 50,
    }).unwrap();
    planet_mesh.duplicate_vertices();
    planet_mesh.compute_flat_normals();

    let moon_radius = planet_radius / 4.0;
    let mut moon_mesh = Mesh::try_from(shape::Icosphere {
        radius: moon_radius,
        subdivisions: 10,
    }).unwrap();
    moon_mesh.duplicate_vertices();
    moon_mesh.compute_flat_normals();

    let sun_radius = moon_radius * 400.0;
    let sun_mesh = Mesh::try_from(shape::Icosphere {
        radius: sun_radius,
        subdivisions: 0,
    }).unwrap();

    let planet_mesh_handle = meshes.add(planet_mesh);
    let moon_mesh_handle = meshes.add(moon_mesh);
    let sun_mesh_handle = meshes.add(sun_mesh);



    let planet_material = StandardMaterial {
        base_color: Color::YELLOW_GREEN,
        reflectance: 0.0,
        ..default()
    };
    let moon_material = StandardMaterial {
        base_color: Color::SILVER,
        reflectance: 0.0,
        ..default()
    };
    let sun_material = StandardMaterial {
        base_color: Color::BLACK,
        emissive: Color::YELLOW,
        reflectance: 0.0,
        ..default()
    };
    let beetlejuice_material = StandardMaterial {
        base_color: Color::RED,
        emissive: Color::RED,
        reflectance: 0.0,
        ..default()
    };
    let alpha_centauri_material = StandardMaterial {
        base_color: Color::CYAN,
        emissive: Color::CYAN,
        reflectance: 0.0,
        ..default()
    };

    let planet_material_handle = materials.add(planet_material);
    let moon_material_handle = materials.add(moon_material);
    let sun_material_handle = materials.add(sun_material);
    let beetlejuice_material_handle = materials.add(beetlejuice_material);
    let alpha_centauri_material = materials.add(alpha_centauri_material);



    com.spawn(
        PbrBundle {
            mesh: planet_mesh_handle,
            material: planet_material_handle,
            transform: Transform::IDENTITY,
            ..default()
        }
    );
    let moon_distance = 500.0;
    com.spawn(
        PbrBundle {
            mesh: moon_mesh_handle,
            material: moon_material_handle,
            transform: Transform::from_xyz(0.0, 0.0, moon_distance),
            ..default()
        }
    );
    let sun_distance = moon_distance * 400.0;
    com.spawn((
        PbrBundle {
            mesh: sun_mesh_handle.clone(),
            material: sun_material_handle.clone(),
            transform: Transform::from_xyz(0.0, sun_distance, 0.0),
            ..default()
        },
        bevy::pbr::NotShadowCaster,
    ));
    let star_distance = sun_distance * 50.0;
    let beetlejuice_size = 10.0;
    com.spawn((
        PbrBundle {
            mesh: sun_mesh_handle.clone(),
            material: beetlejuice_material_handle.clone(),
            transform: Transform::from_xyz(star_distance, 0.0, 0.0)
                .with_scale(Vec3::splat(beetlejuice_size)),
            ..default()
        },
        bevy::pbr::NotShadowCaster,
    ));
    let alpha_centauri_size = 2.0;
    com.spawn((
        PbrBundle {
            mesh: sun_mesh_handle.clone(),
            material: alpha_centauri_material.clone(),
            transform: Transform::from_xyz(-star_distance, 0.0, 0.0)
            .with_scale(Vec3::splat(alpha_centauri_size)),
            ..default()
        },
        bevy::pbr::NotShadowCaster,
    ));
    com.spawn((
        PbrBundle {
            mesh: sun_mesh_handle.clone(),
            material: sun_material_handle.clone(),
            transform: Transform::from_xyz(0.0, -star_distance, 0.0),
            ..default()
        },
        bevy::pbr::NotShadowCaster,
    ));
    com.spawn((
        PbrBundle {
            mesh: sun_mesh_handle.clone(),
            material: sun_material_handle.clone(),
            transform: Transform::from_xyz(0.0, 0.0, -star_distance),
            ..default()
        },
        bevy::pbr::NotShadowCaster,
    ));


    // spawn point light
    com.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, sun_distance, 0.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    com.insert_resource(config);
}