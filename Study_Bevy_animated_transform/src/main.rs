use bevy::{
    animation::{AnimationTarget, AnimationTargetId},
    prelude::*,
};
use std::f32::consts::PI;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, animate_materials)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut animations: ResMut<Assets<AnimationClip>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2., 2.5, 5.).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 500000.,
            ..Default::default()
        },
        transform: Transform::from_xyz(0., 2.5, 0.),
        ..Default::default()
    });

    let planet = Name::new("planet");
    let orbit_controller = Name::new("orbit_controller");
    let satellite = Name::new("satellite");

    let mut animation = AnimationClip::default();
    let planet_animation_target_id = AnimationTargetId::from_name(&planet);

    animation.add_curve_to_target(
        planet_animation_target_id,
        VariableCurve {
            keyframe_timestamps: vec![0.0, 1.0, 2.0, 3.0, 4.0],
            keyframes: Keyframes::Translation(vec![
                Vec3::new(1., 0., 1.),
                Vec3::new(-1., 0., 1.),
                Vec3::new(-1., 0., -1.),
                Vec3::new(1., 0., -1.),
                Vec3::new(1., 0., 1.),
            ]),
            interpolation: Interpolation::Linear,
        },
    );

    let orbit_controller_animation_target_id =
        AnimationTargetId::from_names([planet.clone(), orbit_controller.clone()].iter());

    animation.add_curve_to_target(
        orbit_controller_animation_target_id,
        VariableCurve {
            keyframe_timestamps: vec![0., 1., 2., 3., 4.],
            keyframes: Keyframes::Rotation(vec![
                Quat::IDENTITY,
                Quat::from_axis_angle(Vec3::Y, PI / 2.),
                Quat::from_axis_angle(Vec3::Y, PI),
                Quat::from_axis_angle(Vec3::Y, PI / 2. * 3.),
                Quat::IDENTITY,
            ]),
            interpolation: Interpolation::Linear,
        },
    );

    let satellite_animation_target_id = AnimationTargetId::from_names(
        [planet.clone(), orbit_controller.clone(), satellite.clone()].iter(),
    );
    animation.add_curve_to_target(
        satellite_animation_target_id,
        VariableCurve {
            keyframe_timestamps: vec![0., 0.5, 1., 1.5, 2., 2.5, 3., 3.5, 4.],
            keyframes: Keyframes::Scale(vec![
                Vec3::splat(0.8),
                Vec3::splat(1.2),
                Vec3::splat(0.8),
                Vec3::splat(1.2),
                Vec3::splat(0.8),
                Vec3::splat(0.2),
                Vec3::splat(0.8),
                Vec3::splat(1.8),
                Vec3::splat(0.8),
            ]),
            interpolation: Interpolation::Linear,
        },
    );

    animation.add_curve_to_target(
        AnimationTargetId::from_names(
            [planet.clone(), orbit_controller.clone(), satellite.clone()].iter(),
        ),
        VariableCurve {
            keyframe_timestamps: vec![0., 1., 2., 3., 4.],
            keyframes: Keyframes::Rotation(vec![
                Quat::IDENTITY,
                Quat::from_axis_angle(Vec3::Y, PI / 2.),
                Quat::from_axis_angle(Vec3::Y, PI),
                Quat::from_axis_angle(Vec3::Y, PI / 2. * 3.),
                Quat::IDENTITY,
            ]),
            interpolation: Interpolation::Linear,
        },
    );

    let (graph, animation_index) = AnimationGraph::from_clip(animations.add(animation));

    let mut player = AnimationPlayer::default();
    player.play(animation_index).repeat();
    let planet_entity = commands
        .spawn((
            PbrBundle {
                mesh: meshes.add(Sphere::default()),
                material: materials.add(Color::srgb(0.8, 0.7, 0.6)),
                ..Default::default()
            },
            planet,
            graphs.add(graph),
            player,
        ))
        .id();
    commands
        .entity(planet_entity)
        .insert(AnimationTarget {
            id: planet_animation_target_id,
            player: planet_entity,
        })
        .with_children(|p| {
            p.spawn((
                SpatialBundle::INHERITED_IDENTITY,
                orbit_controller,
                AnimationTarget {
                    id: orbit_controller_animation_target_id,
                    player: planet_entity,
                },
            ))
            .with_children(|p| {
                p.spawn((
                    PbrBundle {
                        transform: Transform::from_xyz(1.5, 0., 0.),
                        mesh: meshes.add(Cuboid::new(0.5, 0.5, 0.5)),
                        material: materials.add(Color::srgb(0.3, 0.9, 0.3)),
                        ..Default::default()
                    },
                    AnimationTarget {
                        id: satellite_animation_target_id,
                        player: planet_entity,
                    },
                    satellite,
                ));
            });
        });
}
fn animate_materials(
    material_handles: Query<&Handle<StandardMaterial>>,
    time: Res<Time>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    input_keyboard: Res<ButtonInput<KeyCode>>,
    mut speed: Local<f32>,
    mut red: Local<f32>,
) {
    if input_keyboard.just_pressed(KeyCode::ArrowUp) {
        *speed += 0.1;
    } else if input_keyboard.just_pressed(KeyCode::ArrowDown) {
        *speed -= 0.1;
    }
    for material_handle in &material_handles {
        // println!("handle chack");
        if let Some(material) = materials.get_mut(material_handle) {
            // println!("material chack");
            if let Color::Srgba(ref mut srgba) = material.base_color {
                *red += *speed * time.delta_seconds();
                let green = srgba.green;
                let blue = srgba.blue;
                *srgba = Srgba::rgb(red.sin(), green, blue);
            }
        }
    }
}
