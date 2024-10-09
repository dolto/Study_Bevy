use std::f32::consts::PI;

use bevy::prelude::*;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

fn main() {
    App::new().add_plugins(DefaultPlugins).run();
}

#[derive(Component, Default)]
struct Cube {
    initial_transform: Transform,
    target_transform: Transform,
    progress: u16,
    in_motion: bool,
}

#[derive(Component)]
struct RandomAxes(Vec3, Vec3);

#[derive(Component)]
struct Instructions;

#[derive(Resource)]
struct MousePressed(bool);

#[derive(Resource)]
struct SeededRng(ChaCha8Rng);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut seeded_rng = ChaCha8Rng::seed_from_u64(rand::thread_rng().gen());

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(3., 2.5, 4.).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    commands.spawn(PbrBundle {
        transform: Transform::from_xyz(0., -2., 0.),
        mesh: meshes.add(Plane3d::default().mesh().size(100., 100.)),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
        ..Default::default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4., 7., -4.),
        ..Default::default()
    });

    let first = random_direction(&mut seeded_rng);
    let second = random_direction(&mut seeded_rng);

    commands.spawn(RandomAxes(first, second));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1., 1., 1.)),
            material: materials.add(Color::rgb(0.5, 0.5, 0.5)),
            ..Default::default()
        },
        Cube {
            initial_transform: Transform::IDENTITY,
            target_transform: random_exes,
        },
    ))
}

fn random_direction(rng: &mut impl Rng) -> Vec3 {
    let height = rng.gen::<f32>() * 2. - 1.;
    let theta = rng.gen::<f32>() * 2. * PI;

    build_direction(height, theta)
}

fn build_direction(height: f32, theta: f32) -> Vec3 {
    let z = height;
    let m = f32::acos(z).sin();
    let x = theta.cos() * m;
    let y = theta.sin() * m;
    Vec3::new(x, y, z)
}

fn random_axes_target_alignment(random_axes: &RandomAxes) -> Transform {
    let RandomAxes(first, second) = random_axes;
    Transform::IDENTITY.aligned_by(Vec3::NEG_Z, *first, Vec3::X, *second)
}
