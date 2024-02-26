use bevy::prelude::*;
use rand::{thread_rng, Rng};
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, draw_cursor)
        .run();
}

#[derive(Component)]
struct Ground;
#[derive(Component)]
struct Object;
fn draw_cursor(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    ground_query: Query<&GlobalTransform, With<Ground>>,
    mut object_query: Query<
        (&GlobalTransform, &mut Handle<StandardMaterial>, &Transform),
        With<Object>,
    >,
    windows: Query<&Window>,
    mut gizmos: Gizmos,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let (camera, camera_transform) = camera_query.single();
    let ground = ground_query.single();

    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };

    let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    let Some(distance) = ray.intersect_plane(ground.translation(), Plane3d::new(ground.up()))
    else {
        return;
    };

    for (obj_transform, mut color, obj_trans) in object_query.iter_mut() {
        let Some(distance) = ray.intersect_plane(
            obj_transform.translation(),
            Plane3d::new(obj_transform.up()),
        ) else {
            continue;
        };

        let point = ray.get_point(distance);
        let trans_distance = point.distance(obj_transform.translation());

        if trans_distance < obj_trans.scale.x * 2. {
            *color = materials.add(Color::rgb(1., 1., 0.));
        } else {
            *color = materials.add(Color::rgb(0.5, 0.5, 0.2));
        }
    }
    let point = ray.get_point(distance);

    gizmos.circle(
        point + ground.up() * 0.01,
        Direction3d::new_unchecked(ground.up()),
        0.2,
        Color::WHITE,
    );
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(20., 20.)),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
            ..default()
        },
        Ground,
    ));

    let mut rand_rng = thread_rng();
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(2., 2., 2.)),
            material: materials.add(Color::rgb(0.5, 0.5, 0.2)),
            transform: Transform::from_xyz(
                rand_rng.gen_range(1.0..5.0),
                1.,
                rand_rng.gen_range(1.0..5.0),
            ),
            ..default()
        },
        Object,
    ));

    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(15.0, 5.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
