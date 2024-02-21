use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, update_playerble_entity)
        .run();
}
const X_EXTENT: f32 = 600.;
#[derive(Component)]
struct Playerble;
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    let shapes = [
        Mesh2dHandle(meshes.add(Circle { radius: 50.0 })),
        Mesh2dHandle(meshes.add(Ellipse::new(25.0, 50.0))),
        Mesh2dHandle(meshes.add(Capsule2d::new(25.0, 50.0))),
        Mesh2dHandle(meshes.add(Rectangle::new(50.0, 100.0))),
        Mesh2dHandle(meshes.add(RegularPolygon::new(50.0, 6))),
        Mesh2dHandle(meshes.add(Triangle2d::new(
            Vec2::Y * 50.0,
            Vec2::new(-50.0, -50.0),
            Vec2::new(50.0, -50.0),
        ))),
    ];
    let num_shapes = shapes.len() as f32;

    for (i, shape) in shapes.into_iter().enumerate() {
        let color = Color::hsl(360. * i as f32 / num_shapes, 0.95, 0.7);
        let mut temp = commands.spawn(MaterialMesh2dBundle {
            mesh: shape,
            material: materials.add(color),
            transform: Transform::from_xyz(
                -X_EXTENT / 2. + i as f32 / (num_shapes - 1.) * X_EXTENT,
                0.,
                0.,
            ),
            ..default()
        });
        if i == 0 {
            temp.insert(Playerble);
        }
    }
}
fn update_playerble_entity(
    mut query_player: Query<&mut Transform, With<Playerble>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let _time = time.delta_seconds();
    for mut trans in query_player.iter_mut() {
        if keyboard.pressed(KeyCode::ArrowUp) {
            trans.translation.y += 100. * _time;
        }
        if keyboard.pressed(KeyCode::ArrowDown) {
            trans.translation.y += -100. * _time;
        }
        if keyboard.pressed(KeyCode::ArrowLeft) {
            trans.translation.x += -100. * _time;
        }
        if keyboard.pressed(KeyCode::ArrowRight) {
            trans.translation.x += 100. * _time;
        }
    }
}
