use bevy::prelude::*;
use std::f32::consts::{PI, TAU};
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_gizmo_group::<MyRoundGizmos>()
        .add_systems(Startup, setup)
        .add_systems(Update, (draw_example_collection, update_config))
        .run();
}

#[derive(Default, Reflect, GizmoConfigGroup)]
struct MyRoundGizmos;

#[derive(Component)]
struct MyGizmoLog;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        MyGizmoLog,
        TextBundle::from_section(
            format!(
                "Hold 'Left' or 'Right' to move of round gizmos\n
                Hold 'up' or 'Down' to move of line gizmos\n
                Press '1' or '2' to toggle the visibility of straight gizmos or round gizmos"
            ),
            TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: 24.,
                color: Color::WHITE,
            },
        ),
    ));
}
fn draw_example_collection(
    mut gizmos: Gizmos,
    mut my_gizmos: Gizmos<MyRoundGizmos>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut local_sin: Local<f32>,
    mut local_tau: Local<f32>,
) {
    let left = if keyboard.pressed(KeyCode::ArrowLeft) {
        1.
    } else {
        0.
    };
    let right = if keyboard.pressed(KeyCode::ArrowRight) {
        -1.
    } else {
        0.
    };
    let down = if keyboard.pressed(KeyCode::ArrowDown) {
        -1.
    } else {
        0.
    };
    let up = if keyboard.pressed(KeyCode::ArrowUp) {
        1.
    } else {
        0.
    };

    *local_sin += left * time.delta_seconds() + right * time.delta_seconds();
    *local_tau += up * time.delta_seconds() + down * time.delta_seconds();

    let sin = local_sin.sin() * 50.;
    let tau = *local_tau % TAU;
    gizmos.line_2d(Vec2::Y * -sin, Vec2::splat(-80.), Color::RED);
    gizmos.line_2d(Vec2::splat(80.), Vec2::Y * sin, Color::GREEN);

    gizmos.linestrip_gradient_2d([
        (Vec2::Y * 300., Color::BLUE),
        (Vec2::new(-255., -155.), Color::GREEN),
        (Vec2::new(255., -155.), Color::RED),
        (Vec2::Y * 300., Color::BLUE),
    ]);

    gizmos.rect_2d(Vec2::ZERO, tau, Vec2::splat(300.), Color::YELLOW_GREEN);

    my_gizmos
        .circle_2d(Vec2::ZERO, 300., Color::NAVY)
        .segments(64);
    my_gizmos.ellipse_2d(Vec2::ZERO, tau * -1., Vec2::new(100., 200.), Color::GRAY);
    my_gizmos.arc_2d(Vec2::ZERO, sin / 10., PI / 2., 350., Color::ORANGE_RED);

    gizmos.arrow_2d(
        Vec2::ZERO,
        Vec2::from_angle(sin / -10. + PI / 2.) * 50.,
        Color::YELLOW,
    );
}
fn update_config(mut config_store: ResMut<GizmoConfigStore>, keyboard: Res<ButtonInput<KeyCode>>) {
    let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();
    if keyboard.just_pressed(KeyCode::Digit1) {
        config.enabled ^= true;
    }

    let (my_config, _) = config_store.config_mut::<MyRoundGizmos>();
    if keyboard.just_pressed(KeyCode::Digit2) {
        my_config.enabled ^= true;
    }
}
