use bevy::pbr::{CascadeShadowConfigBuilder, CascadeShadowConfig};
use bevy::prelude::*;

use rand::prelude::*;
use rand_pcg::Pcg64;
use rand_seeder::Seeder;

use std::ops::Range;
use std::f32::consts::PI;

use crate::star_names::STAR_NAMES;

pub struct PlanetPlugin;

impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, startup);
    }
}

#[derive(Clone)]
pub struct Body {
    pub radius: f32,
    pub translation: Vec3,
    material: StandardMaterial,
}

impl Body {

    fn new(radius: f32, color: Color, emissive: bool) -> Self {

        let translation = Vec3::ZERO;

        let material;
        if emissive {
            material = StandardMaterial {
                base_color: Color::BLACK,
                emissive: color,
                reflectance: 0.0,
                ..default()
            };
        } else {
            material = StandardMaterial {
                base_color: color,
                ..default()
            };
        }

        Body {
            radius,
            translation,
            material,
        }
    }

    fn with_distance_from(mut self, distance: f32, distance_axis: Vec3, from: Vec3) -> Self {
        let translation = from + distance_axis * distance;
        self.translation = translation;
        self
    }

}

const RED_STAR: Color = Color::rgb(0.996, 0.494, 0.137);
const YELLOW_STAR: Color = Color::rgb(0.996, 0.855, 0.714);
const BLUE_STAR: Color = Color::rgb(0.604, 0.686, 0.996);

const STAR_COLORS: [Color; 3] = [RED_STAR, YELLOW_STAR, BLUE_STAR];
const STARLIGHT_INTENSITY: f32 = 3.4028235e13;

const GRASS: Color = Color::YELLOW_GREEN;
const DUST: Color = Color::SILVER;

pub const AU: f32 = 400_000.0;
const EARTH_RADIUS: f32 = 100.0;
const SOL_RADIUS: f32 = EARTH_RADIUS * 100.0;
const MOON_RADIUS: f32 = EARTH_RADIUS / 4.0;
const MOON_DISTANCE: f32 = AU / 400.0;

#[derive(Resource, Clone)]
pub struct Config {
    clear_color: Color,
    galaxy_size: f32,
    star_radius: Range<f32>,
    star_separation: f32,

    // PlayerPlugin needs to know where it can walk
    pub planet: Body,
    pub moon: Body,
}

impl Default for Config {
    fn default() -> Self {

        let planet = Body::new(
            EARTH_RADIUS,
            GRASS,
            false,
        );

        let moon = Body::new(
            MOON_RADIUS,
            DUST,
            false,
        )
        .with_distance_from(MOON_DISTANCE, Vec3::Z, planet.translation);
   
        Config {
            clear_color: Color::rgb(0.1, 0.1, 0.1),
            galaxy_size: AU * 200.0,
            star_radius: SOL_RADIUS / 10.0..SOL_RADIUS * 10.0,
            star_separation: AU * 20.0,

            planet,
            moon,
        }
    }
}



struct Moon {
    name: String,
    orbit: Vec3,
    radius: f32,
    color: Color,
}

struct Planet {
    name: String,
    orbit: Vec3,
    radius: f32,
    color: Color,
    moons: Vec<Moon>,
}

struct Star {
    name: String,
    translation: Vec3,
    radius: f32,
    color: Color,
    planets: Vec<Planet>,
}

struct Galaxy {
    name: String,
    stars: Vec<Star>,
}

impl Galaxy {
    fn new(name: String, config: &Config) -> Self {
        let mut rng: Pcg64 = Seeder::from(name.clone()).make_rng();

        let mut stars: Vec<Star> = Vec::new();

        stars.push(Star {
            name: "Sol".to_string(),
            translation: Vec3::Y * AU,
            radius: SOL_RADIUS,
            color: YELLOW_STAR,
            planets: Vec::new(),
        });

        for star in STAR_NAMES.iter() {

            // generate a random rotation as a quaternion
            let rotation =
                Quat::from_rotation_x(rng.gen_range(-PI..PI)) *
                Quat::from_rotation_y(rng.gen_range(-PI..PI)) *
                Quat::from_rotation_z(rng.gen_range(-PI..PI));

            let distance = rng.gen_range(0.0..config.galaxy_size) * Vec3::Y;

            let mut translation = rotation * distance;
            // check that translation is not within 2 * AU of any other star
            for other_star in stars.iter() {
                if (translation - other_star.translation).length() < config.star_separation {
                    // move the star 2.0 * AU away from the other star
                    translation += (translation - other_star.translation).normalize() * config.star_separation;
                }
            }

            let radius = rng.gen_range(config.star_radius.clone());

            let color = STAR_COLORS[rng.gen_range(0..STAR_COLORS.len())];

            stars.push(Star {
                name: star.to_string(),
                translation,
                radius,
                color,
                planets: Vec::new(),
            });
        }


        Galaxy {
            name,
            stars,
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

    let galaxy = Galaxy::new("Milky Way".to_string(), &config);

    let mesh_handle = meshes.add(
        Mesh::try_from(shape::Icosphere {
            radius: 1.0,
            subdivisions: 40,
        }).unwrap()
    );

    let cascade_shadow_config: CascadeShadowConfig = CascadeShadowConfigBuilder {
        maximum_distance: config.star_separation,
        ..default()
    }.into();

    for star in galaxy.stars {
        com.spawn((
            PbrBundle {
                mesh: mesh_handle.clone(),
                material: materials.add(
                    StandardMaterial {
                        base_color: Color::BLACK,
                        emissive: star.color,
                        reflectance: 0.0,
                        ..default()
                    }
                ),
                transform: Transform::from_translation(star.translation)
                    .with_scale(Vec3::splat(star.radius)),
                ..default()
            },
            bevy::pbr::NotShadowCaster,
        ));
        // com.spawn((
        //     PointLightBundle {
        //         point_light: PointLight {
        //             color: star.color,
        //             intensity: STARLIGHT_INTENSITY,
        //             range: config.galaxy_size,
        //             radius: star.radius,
        //             shadows_enabled: true,
        //             ..default()
        //         },
        //         transform: Transform::from_translation(star.translation),
        //         ..default()
        //     },
        //     cascade_shadow_config.clone(),
        // ));
    }

    // Planet
    com.spawn(PbrBundle {
            mesh: mesh_handle.clone(),
            material: materials.add(config.planet.material),
            transform: Transform::from_translation(config.planet.translation)
                .with_scale(Vec3::splat(config.planet.radius)),
            ..default()
    });

    // Moon
    com.spawn(PbrBundle {
            mesh: mesh_handle.clone(),
            material: materials.add(config.moon.material),
            transform: Transform::from_translation(config.moon.translation)
                .with_scale(Vec3::splat(config.moon.radius)),
            ..default()
    });

    // let sun_light = DirectionalLight {
    //     color: YELLOW_STAR,
    //     shadows_enabled: true,
    //     ..default()
    // };
    // com.spawn(DirectionalLightBundle {
    //     directional_light: sun_light,
    //     transform: Transform::from_translation(Vec3::Y * AU)
    //         .looking_at(config.planet.translation, Vec3::Y),
    //     ..default()
    // });

    let sun_point_light = PointLight {
        color: YELLOW_STAR,
        intensity: STARLIGHT_INTENSITY,
        range: AU * 2.0,
        radius: SOL_RADIUS,
        shadows_enabled: true,
        ..default()
    };
    com.spawn(PointLightBundle {
        point_light: sun_point_light,
        transform: Transform::from_translation(Vec3::Y * 125.00),
        ..default()
    });
}