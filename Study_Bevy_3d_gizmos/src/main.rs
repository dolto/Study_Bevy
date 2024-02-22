use bevy::prelude::*;
use std::f32::consts::PI;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<MovementValue>()
        .init_gizmo_group::<MyRoundGizmos>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (rotate_camera, draw_example_collection, update_config),
        )
        .run();
}
#[derive(Default, Reflect, GizmoConfigGroup)]
struct MyRoundGizmos;

#[derive(Resource)]
struct MovementValue {
    move1: f32,
    move2: f32,
}
impl Default for MovementValue {
    fn default() -> Self {
        MovementValue {
            move1: 0.,
            move2: 0.,
        }
    }
}
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 1.5, 6.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(5., 5.)),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6)),
        transform: Transform::from_xyz(0., 0.5, 0.),
        ..default()
    });
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

fn rotate_camera(mut query: Query<&mut Transform, With<Camera>>, time: Res<Time>) {
    let mut transform = query.single_mut();

    transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(time.delta_seconds() / 2.));
}

fn draw_example_collection(
    mut gizmos: Gizmos,
    mut my_gizmos: Gizmos<MyRoundGizmos>,
    movement_value: Res<MovementValue>,
) {
    let move1 = movement_value.move1;
    let move2 = movement_value.move2;
    gizmos.cuboid(
        Transform::from_translation(Vec3::Y * 0.5).with_scale(Vec3::splat(1.25)),
        Color::GRAY,
    );
    gizmos.rect(
        Vec3::new(move1.cos() * 2.5, 1., 0.),
        Quat::from_rotation_y(PI / 2.),
        Vec2::splat(2.),
        Color::GREEN,
    );

    my_gizmos.sphere(Vec3::new(1., 0.5, 0.), Quat::IDENTITY, 0.5, Color::RED);

    for y in [0., 0.5, 1.] {
        gizmos.ray(
            Vec3::new(1., y, 0.),
            Vec3::new(-3., (move1 * 3.).sin(), 0.),
            Color::BLUE,
        );
    }
    my_gizmos
        .arc_3d(
            180.0_f32.to_radians(),
            0.2,
            Vec3::ONE,
            Quat::from_rotation_arc(Vec3::Y, Vec3::ONE.normalize()),
            Color::ORANGE,
        )
        .segments(10);
    my_gizmos.circle(Vec3::ZERO, Direction3d::Y, 3., Color::DARK_GRAY);
    my_gizmos
        .circle(Vec3::ZERO, Direction3d::Y, 3.1 + move2, Color::NAVY)
        .segments(64);
    my_gizmos.sphere(Vec3::ZERO, Quat::IDENTITY, 3.2, Color::BLACK);

    gizmos.arrow(Vec3::ZERO, Vec3::ONE * 1.5, Color::YELLOW);
}
fn update_config(
    mut config_store: ResMut<GizmoConfigStore>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut movement_value: ResMut<MovementValue>,
    time: Res<Time>,
) {
    if keyboard.just_pressed(KeyCode::KeyD) {
        for (_, config, _) in config_store.iter_mut() {
            config.depth_bias = if config.depth_bias == 0. { -1. } else { 0. };
        }
    }

    if keyboard.just_pressed(KeyCode::KeyP) {
        for (_, config, _) in config_store.iter_mut() {
            config.line_perspective ^= true;
            config.line_width *= if config.line_perspective { 5. } else { 1. / 5. };
        }
    }
    let delta_time = time.delta_seconds();
    let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();
    if keyboard.pressed(KeyCode::ArrowRight) {
        config.line_width += 5. * delta_time;
        config.line_width = config.line_width.clamp(0., 50.);
    }
    if keyboard.pressed(KeyCode::ArrowLeft) {
        config.line_width -= 5. * delta_time;
        config.line_width = config.line_width.clamp(0., 50.);
    }
    if keyboard.just_pressed(KeyCode::Digit1) {
        config.enabled ^= true;
    }
    let (my_config, _) = config_store.config_mut::<MyRoundGizmos>();

    if keyboard.pressed(KeyCode::ArrowUp) {
        my_config.line_width += 5. * delta_time;
        my_config.line_width = my_config.line_width.clamp(0., 50.);
    }
    if keyboard.pressed(KeyCode::ArrowDown) {
        my_config.line_width -= 5. * delta_time;
        my_config.line_width = my_config.line_width.clamp(0., 50.);
    }
    if keyboard.just_pressed(KeyCode::Digit2) {
        my_config.enabled ^= true;
    }
    if keyboard.just_pressed(KeyCode::KeyA) {
        config_store.config_mut::<AabbGizmoConfigGroup>().1.draw_all ^= true;
    }
    if keyboard.pressed(KeyCode::ShiftLeft) {
        movement_value.move1 += 10. * delta_time;
    }
    if keyboard.pressed(KeyCode::ShiftRight) {
        movement_value.move1 += -10. * delta_time;
    }
    if keyboard.pressed(KeyCode::KeyN) {
        movement_value.move2 += 10. * delta_time;
    }
    if keyboard.pressed(KeyCode::KeyM) {
        movement_value.move2 += -10. * delta_time;
    }
}
