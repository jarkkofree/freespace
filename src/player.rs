use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::CursorGrabMode;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, startup)
            .add_systems(Update, (
                mouse_control,
                walk_on,
            ));
    }
}

#[derive(Resource)]
struct Config {
    look_sensitivity: f32,
    walk_speed: f32,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            look_sensitivity: 0.001,
            walk_speed: 10.0,
        }
    }
}

fn startup(
    mut com: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let config = Config::default();
    let planet_config = crate::planet::Config::default();

    let leg_height = 1.0;
    let torso_height = 1.0;
    let head_height = 0.4;

    let leg_shape = shape::Box::new(0.6, leg_height, 0.3);
    let torso_shape = shape::Box::new(1.0, torso_height, 0.5);
    let head_shape = shape::Box::new(0.4, head_height, 0.5);

    let leg_mesh = Mesh::from(leg_shape);
    let torso_mesh = Mesh::from(torso_shape);
    let head_mesh = Mesh::from(head_shape);


    let leg_mesh_handle = meshes.add(leg_mesh);
    let torso_mesh_handle = meshes.add(torso_mesh);
    let head_mesh_handle = meshes.add(head_mesh);

    let leg_color = Color::BEIGE;
    let torso_color = Color::CYAN;
    let head_color = Color::SALMON;

    let leg_material = StandardMaterial::from(leg_color);
    let torso_material = StandardMaterial::from(torso_color);
    let head_material = StandardMaterial::from(head_color);

    let leg_material_handle = materials.add(leg_material);
    let torso_material_handle = materials.add(torso_material);
    let head_material_handle = materials.add(head_material);

    const HALF : f32 = 0.5;
    let leg_offset = leg_height * HALF;
    let torso_offset = leg_height * HALF + torso_height * HALF;
    let head_offset = torso_height * HALF + head_height * HALF + 0.1;


    let leg_transform = Transform::from_xyz(0.0, leg_offset, 0.0);
    let torso_transform = Transform::from_xyz(0.0, torso_offset, 0.0);
    let head_transform = Transform::from_xyz(0.0, head_offset, 0.0);

    let body_translation = planet_config.planet.translation;
    let body_radius = planet_config.planet.radius;
    let body_contact_translation = body_translation + Vec3::new(0.0, body_radius, 0.0);
    let body_contact = com.spawn((
        PbrBundle {
            transform: Transform::from_translation(body_contact_translation),
            ..default()
        },
        PlanetContact,
        OnPlanet {
            planet: Transform::from_translation(body_translation),
            radius: body_radius,
        },
    )).id();
    let legs = com.spawn((
        PbrBundle {
            mesh: leg_mesh_handle,
            material: leg_material_handle,
            transform: leg_transform,
            ..default()
        },
        Legs,
    )).id();
    let torso = com.spawn((
        PbrBundle {
            mesh: torso_mesh_handle,
            material: torso_material_handle,
            transform: torso_transform,
            ..default()
        },
        Torso
    )).id();
    let head = com.spawn((
        PbrBundle {
            mesh: head_mesh_handle,
            material: head_material_handle,
            transform: head_transform,
            ..default()
        },
    )).id();
    let camera = com.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 7.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }).id();

    com.entity(body_contact).push_children(&[legs]);
    com.entity(legs).push_children(&[torso]);
    com.entity(torso).push_children(&[head]);
    com.entity(head).push_children(&[camera]);

    com.insert_resource(config);
}



#[derive(Component)]
struct Legs; // can yaw left and right

#[derive(Component)]
struct Torso; // can pitch up and down

fn mouse_control(
    mut legs: Query<&mut Transform, (With<Legs>, Without<Torso>)>,
    mut torso: Query<&mut Transform, (With<Torso>, Without<Legs>)>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut window: Query<&mut Window>,
    mouse: Res<Input<MouseButton>>,
    config: Res<Config>,
) {
    let mut window = window.single_mut();
    let mut legs = legs.single_mut();
    let mut torso = torso.single_mut();

    if mouse.just_pressed(MouseButton::Left) {
        window.cursor.visible = false;
        window.cursor.grab_mode = CursorGrabMode::Locked;
    }

    if mouse.just_pressed(MouseButton::Right) {
        window.cursor.visible = true;
        window.cursor.grab_mode = CursorGrabMode::None;
    }

    if window.cursor.grab_mode == CursorGrabMode::Locked {
        let mut total_delta_x = 0.0;
        let mut total_delta_y = 0.0;
        // Accumulate deltas from all events.
        for mouse_motion in mouse_motion_events.read() {
            total_delta_x -= mouse_motion.delta.x;
            total_delta_y -= mouse_motion.delta.y;
        }
        // check if total_delta is not zero
        if total_delta_x != 0.0 || total_delta_y != 0.0 {

            let rotation_input = total_delta_x * config.look_sensitivity;
            legs.rotate(Quat::from_rotation_y(rotation_input));

            let rotation_input = total_delta_y * config.look_sensitivity;
            torso.rotate(Quat::from_rotation_x(rotation_input));

        }
    }
}

#[derive(Component)]
struct PlanetContact; // always on the ground

#[derive(Component)]
struct OnPlanet {
    planet: Transform,
    radius: f32,
}

fn walk_on(
    mut planet_contact: Query<(&mut Transform, &OnPlanet), (With<PlanetContact>, Without<Legs>)>,
    mut legs: Query<&GlobalTransform, (With<Legs>, Without<PlanetContact>)>,
    keys: ResMut<Input<KeyCode>>,
    time: Res<Time>,
    config: Res<Config>,
) {
    let (mut planet_contact, on) = planet_contact.single_mut();
    let global_legs_transform = legs.single_mut();

    let mut speed = Vec3::ZERO;

    // get vectors of forward, back, left, right
    if keys.pressed(KeyCode::W) {
        speed += global_legs_transform.forward();
    }
    if keys.pressed(KeyCode::S) {
        speed += global_legs_transform.back();
    }
    if keys.pressed(KeyCode::A) {
        speed += global_legs_transform.left();
    }
    if keys.pressed(KeyCode::D) {
        speed += global_legs_transform.right();
    }

    if speed.length_squared() > 0.0 {

        // move feet
        let movement = speed.normalize() * config.walk_speed * time.delta_seconds();
        planet_contact.translation += movement;

        // keep feet on the ground
        let radius = on.radius;
        let relative_position = planet_contact.translation - on.planet.translation;
        planet_contact.translation = on.planet.translation + relative_position.normalize() * radius;

        // realign legs to planet up
        let up_difference = Quat::from_rotation_arc(
            global_legs_transform.up(), 
            (planet_contact.translation - on.planet.translation).normalize()
        );

        // Apply the rotation difference to the player
        planet_contact.rotate(up_difference);

    }
}
