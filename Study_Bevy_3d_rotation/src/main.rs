use bevy::prelude::*;
use std::f32::consts::TAU;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (rotate_cube, control_cube))
        .run();
}

#[derive(Component)]
struct Rotatable {
    speed: f32,
}
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::default()),
            material: materials.add(Color::WHITE),
            transform: Transform::from_translation(Vec3::ZERO),
            ..default()
        },
        Rotatable { speed: 0.3 },
    ));

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 10., 20.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(3., 3., 3.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
fn rotate_cube(mut cubes: Query<(&mut Transform, &Rotatable)>, timer: Res<Time>) {
    for (mut transform, cube) in &mut cubes {
        transform.rotate_local_y(cube.speed * TAU * timer.delta_seconds());
    }
}
fn control_cube(
    mut cubes: Query<(&mut Rotatable, &mut Transform)>,
    keyboard: Res<ButtonInput<KeyCode>>,
    timer: Res<Time>,
) {
    if keyboard.pressed(KeyCode::KeyQ) {
        for (mut cube, _) in cubes.iter_mut() {
            cube.speed += timer.delta_seconds() * 3.;
        }
    }
    if keyboard.pressed(KeyCode::KeyW) {
        for (mut cube, _) in cubes.iter_mut() {
            cube.speed -= timer.delta_seconds() * 3.;
        }
    }
    if keyboard.pressed(KeyCode::KeyM) {
        for (_, mut trans) in cubes.iter_mut() {
            *trans = Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y);
        }
    }
    if keyboard.pressed(KeyCode::KeyN) {
        for (_, mut trans) in cubes.iter_mut() {
            *trans = Transform::from_translation(Vec3::ONE).looking_to(Vec3::ZERO, Vec3::Y);
        }
    }
}
