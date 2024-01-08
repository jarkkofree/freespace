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
    player_spawn: Vec3,
    look_sensitivity: f32,
    walk_speed: f32,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            player_spawn: Vec3::new(0.0, 100.0, 0.0),
            look_sensitivity: 0.001,
            walk_speed: 50.0,
        }
    }
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

fn startup(
    mut com: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let config = Config::default();

    let cube = shape::Box::new(1.0, 1.0, 1.0);
    let mesh = Mesh::from(cube);
    let mesh_handle = meshes.add(mesh);
    let color = Color::SALMON;
    let material = StandardMaterial::from(color);
    let material_handle = materials.add(material);


    let feet = com.spawn((
        PbrBundle {
            transform: Transform::from_xyz(0.0, 100.0, 0.0),
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
        Torso
    )).id();
    let head = com.spawn((
        make_box(&mut meshes, &mut materials, 0.4, 0.4, 0.5, Color::SALMON, 0.8),
    )).id();
    let camera = com.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 7.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }).id();

    com.entity(feet).push_children(&[legs]);
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
struct Feet; // always on the ground

fn walk(
    mut feet: Query<&mut Transform, (With<Feet>, Without<Legs>)>,
    mut legs: Query<&GlobalTransform, (With<Legs>, Without<Feet>)>,
    keys: ResMut<Input<KeyCode>>,
    time: Res<Time>,
    config: Res<Config>,
) {
    let mut feet_transform = feet.single_mut();
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

        // move feet forward
        let movement = speed.normalize() * config.walk_speed * time.delta_seconds();
        feet_transform.translation += movement;

        // keep feet on the ground
        let radius = 100.0;
        let relative_position = feet_transform.translation;
        feet_transform.translation = relative_position.normalize() * radius;

        // 
        let up_difference = Quat::from_rotation_arc(
            global_legs_transform.up(), 
            feet_transform.translation.normalize()
        );

        // Apply the rotation difference to the player
        feet_transform.rotate(up_difference);

    }
}
