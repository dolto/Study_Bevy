use bevy::prelude::*;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_gizmo_group::<MyGizmosGroup>()
        .add_systems(Startup, setup)
        .add_systems(Update, (draw_cursor, update_gizmo_config))
        .run();
}
#[derive(Default, Reflect, GizmoConfigGroup)]
struct MyGizmosGroup;
fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
fn draw_cursor(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mut gizmos: Gizmos,
    mut my_gizmos: Gizmos<MyGizmosGroup>,
) {
    let (camera, camera_transform) = camera_query.single();

    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };

    let Some(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };
    let mirror_point = Vec2::new(point.x * -1., point.y * -1.);

    gizmos.circle_2d(point, 10., Color::WHITE);
    my_gizmos.circle_2d(mirror_point, 10., Color::RED);
}
fn update_gizmo_config(
    mut gizmos_config: ResMut<GizmoConfigStore>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let (gizmo_conf, _) = gizmos_config.config_mut::<DefaultGizmoConfigGroup>();
    if keyboard.just_pressed(KeyCode::Digit1) {
        gizmo_conf.enabled ^= true;
    }

    let (my_gizmo_conf, _) = gizmos_config.config_mut::<MyGizmosGroup>();
    if keyboard.just_pressed(KeyCode::Digit2) {
        my_gizmo_conf.enabled ^= true;
    }
}
