use bevy::prelude::*;

use crate::star;

pub struct PlanetPlugin;

impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, startup);
    }
}


fn get_subdivisions(radius: f32) -> usize {
    let a: f32 = 120.0;
    (80.0-a*80.0/(radius+a)) as usize
}

#[derive(Clone)]
pub struct Body {
    pub radius: f32,
    pub translation: Vec3,
    material: StandardMaterial,
    mesh: Mesh,
}

impl Body {

    fn new(radius: f32, color: Color, emissive: bool) -> Self {

        let translation = Vec3::ZERO;

        let material;
        let subdivisions;
        if emissive {
            material = StandardMaterial {
                base_color: Color::BLACK,
                emissive: color,
                reflectance: 0.0,
                ..default()
            };
            subdivisions = 0; // won't walk on stars, so no need for subdivisions
        } else {
            material = StandardMaterial {
                base_color: color,
                ..default()
            };
            subdivisions = get_subdivisions(radius);
        }

        let shape = shape::Icosphere {
            radius: radius,
            subdivisions,
        };
        let mesh = Mesh::try_from(shape).unwrap();

        Body {
            radius,
            translation,
            material,
            mesh,
        }
    }

    fn with_distance_from(mut self, distance: f32, distance_axis: Vec3, from: Vec3) -> Self {
        let translation = from + distance_axis * distance;
        self.translation = translation;
        self
    }

}

#[derive(Resource, Clone)]
pub struct Config {
    clear_color: Color,
    sun: Body,
    beetlejuice: Body,
    alpha_centauri: Body,

    // PlayerPlugin needs to know where it can walk
    pub planet: Body,
    pub moon: Body,
}

impl Default for Config {
    fn default() -> Self {

        let distance = 100_000.0;

        let planet = Body::new(
            100.0,
            Color::YELLOW_GREEN,
            false,
        );
        let sun = Body::new(
            planet.radius * 100.0,
            Color::YELLOW,
            true,
        )
        .with_distance_from(distance, Vec3::Y, planet.translation);

        let moon = Body::new(
            planet.radius / 4.0,
            Color::SILVER,
            false,
        )
        .with_distance_from(distance / 400.0, Vec3::Z, planet.translation);

        let star_distance = sun.radius * 1_000.0;

        let beetlejuice = Body::new(
            sun.radius * 10.0,
            Color::RED,
            true,
        )
        .with_distance_from(star_distance, Vec3::X, sun.translation);

        let alpha_centauri = Body::new(
            sun.radius * 2.0,
            Color::CYAN,
            true,
        )
        .with_distance_from(star_distance, -Vec3::X, sun.translation);        



        Config {
            clear_color: Color::rgb(0.1, 0.1, 0.1),
            sun,
            planet,
            moon,
            beetlejuice,
            alpha_centauri,
        }
    }
}

fn startup(
    mut com: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let config = Config::default();
    com.insert_resource(config.clone());

    com.insert_resource(ClearColor(config.clear_color));

    // Planet
    com.spawn(PbrBundle {
            mesh: meshes.add(config.planet.mesh),
            material: materials.add(config.planet.material),
            transform: Transform::from_translation(config.planet.translation),
            ..default()
    });

    // Moon
    com.spawn(PbrBundle {
            mesh: meshes.add(config.moon.mesh),
            material: materials.add(config.moon.material),
            transform: Transform::from_translation(config.moon.translation),
            ..default()
    });

    // Sun
    com.spawn((
        PbrBundle {
            mesh: meshes.add(config.sun.mesh),
            material: materials.add(config.sun.material),
            transform: Transform::from_translation(config.sun.translation),
            ..default()
        },
        bevy::pbr::NotShadowCaster,
    ));
    let sun_light = DirectionalLight {
        color: Color::WHITE,
        shadows_enabled: true,
        ..default()
    };
    com.spawn(DirectionalLightBundle {
        directional_light: sun_light,
        transform: Transform::from_translation(config.sun.translation)
            .looking_at(config.planet.translation, Vec3::Y),
        ..default()
    });

    // Stars
    com.spawn((
        PbrBundle {
            mesh: meshes.add(config.beetlejuice.mesh),
            material: materials.add(config.beetlejuice.material),
            transform: Transform::from_translation(config.beetlejuice.translation),
            ..default()
        },
    ));
    com.spawn((
        PbrBundle {
            mesh: meshes.add(config.alpha_centauri.mesh),
            material: materials.add(config.alpha_centauri.material),
            transform: Transform::from_translation(config.alpha_centauri.translation),
            ..default()
        },
    ));
}