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
                walk,
            ));
    }
}

#[derive(Resource)]
struct Config {
    clear_color: Color,
    player_spawn: Vec3,
    look_sensitivity: f32,
    walk_speed: f32,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            clear_color: Color::rgb(0.1, 0.1, 0.1),
            player_spawn: Vec3::new(0.0, 100.0, 0.0),
            look_sensitivity: 0.001,
            walk_speed: 50.0,
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

    let feet = com.spawn((
        PbrBundle {
            transform: Transform::from_translation(config.player_spawn),
            ..default()
        },
        Feet,
    )).id();
    let legs = com.spawn((
        make_box(&mut meshes, &mut materials, 1.0, 0.6, 0.3, Color::BEIGE, 0.5),
        Legs,
    )).id();
    let torso = com.spawn((
        make_box(&mut meshes, &mut materials, 1.0, 1.0, 0.5, Color::CYAN, 1.0),
        Torso,
    )).id();
    let head = com.spawn(
        make_box(&mut meshes, &mut materials, 0.4, 0.4, 0.5, Color::SALMON, 0.8)
    ).id();
    let camera = com.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 7.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }).id();

    com.entity(feet).push_children(&[legs]);
    com.entity(legs).push_children(&[torso]);
    com.entity(torso).push_children(&[head]);
    com.entity(head).push_children(&[camera]);


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



fn make_box(
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    height: f32,
    width: f32,
    depth: f32,
    color: Color,
    y_offset: f32,
) -> PbrBundle {

    let mesh = Mesh::from(shape::Box::new(width, height, depth));
    let material = StandardMaterial::from(color);

    PbrBundle {
        mesh: meshes.add(mesh),
        material: materials.add(material),
        transform: Transform::from_xyz(0.0, y_offset, 0.0),
        ..default()
    }
}

#[derive(Component)]
struct Legs;

#[derive(Component)]
struct Torso;

fn mouse_control(
    mut legs: Query<&mut Transform, (With<Legs>, Without<Torso>)>,
    mut torso: Query<&mut Transform, (With<Torso>, Without<Legs>)>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut window: Query<&mut Window>,
    mouse: Res<Input<MouseButton>>,
    config: Res<Config>,
) {
    let mut window = window.single_mut();
    let mut legs_transform = legs.single_mut();
    let mut torso_transform = torso.single_mut();

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
            legs_transform.rotate(Quat::from_rotation_y(rotation_input));

            let rotation_input = total_delta_y * config.look_sensitivity;
            torso_transform.rotate(Quat::from_rotation_x(rotation_input));

        }
    }
}

#[derive(Component)]
struct Feet;

fn walk(
    mut feet: Query<&mut Transform, (With<Feet>, Without<Legs>)>,
    mut legs: Query<(&mut Transform, &GlobalTransform), (With<Legs>, Without<Feet>)>,
    keys: ResMut<Input<KeyCode>>,
    time: Res<Time>,
    config: Res<Config>,
) {
    let mut feet_transform = feet.single_mut();
    let (mut legs_transform, global_legs_transform) = legs.single_mut();

    let mut speed = Vec3::ZERO;

    if keys.pressed(KeyCode::W) {
        speed += legs_transform.forward();
    }
    if keys.pressed(KeyCode::S) {
        speed += legs_transform.back();
    }
    if keys.pressed(KeyCode::A) {
        speed += legs_transform.left();
    }
    if keys.pressed(KeyCode::D) {
        speed += legs_transform.right();
    }

    if speed.length_squared() > 0.0 {

        let mut movement = speed.normalize() * config.walk_speed * time.delta_seconds();

        let global_up = Transform::IDENTITY.up();
        info!("global up: {:?}", global_up);
        let legs_up = global_legs_transform.up();
        info!("legs up: {:?}", legs_up);

        let up_difference = Quat::from_rotation_arc(
            legs_up, 
            global_up
        );

        info!("rotation: {:?}", up_difference);

        info!("before rotation: {:?}", movement);
        movement = up_difference.mul_vec3(movement);
        info!("after rotation: {:?}", movement);

        feet_transform.translation += movement;

        let radius = 100.0;
        let relative_position = feet_transform.translation;
        feet_transform.translation = relative_position.normalize() * radius;

        let up_difference = Quat::from_rotation_arc(
            global_legs_transform.up(), 
            feet_transform.translation.normalize()
        );

        // Apply the rotation difference to the player
        legs_transform.rotate(up_difference);
    }
}
