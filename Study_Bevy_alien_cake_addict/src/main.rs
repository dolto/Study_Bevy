use std::{collections::btree_map::Range, default, f32::consts::PI};

use bevy::prelude::*;
use rand::{rngs::ThreadRng, thread_rng, Rng};
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Game>()
        .insert_resource(BonusSpawnTimer(Timer::from_seconds(
            5.,
            TimerMode::Repeating,
        )))
        .init_state::<GameState>()
        .add_systems(Startup, setup_cameras)
        .add_systems(OnEnter(GameState::Playing), setup)
        .add_systems(
            Update,
            (
                move_player,
                focus_camera,
                rotate_bonus,
                scoreboard_system,
                spawn_bonus,
                player_pos_set,
                tile_set,
            )
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(OnExit(GameState::Playing), teardown)
        .add_systems(OnEnter(GameState::GameOver), display_score)
        .add_systems(
            Update,
            gameover_keyboard.run_if(in_state(GameState::GameOver)),
        )
        .add_systems(OnExit(GameState::GameOver), teardown)
        .run();
}

#[derive(Clone, PartialEq, Eq, Debug, Hash, Default, States)]
enum GameState {
    #[default]
    Playing,
    GameOver,
}

#[derive(Resource)]
struct BonusSpawnTimer(Timer);

struct Cell {
    height: f32,
    shake: (f32, f32),
    is_up: bool,
    entity: Entity,
}

#[derive(Default)]
struct Player {
    entity: Option<Entity>,
    i: usize,
    j: usize,
    move_cooldown: Timer,
}

#[derive(Default)]
struct Bonus {
    entity: Option<Entity>,
    i: usize,
    j: usize,
    handle: Handle<Scene>,
}

#[derive(Resource, Default)]
struct Game {
    board: Vec<Vec<Cell>>,
    player: Player,
    bonus: Bonus,
    score: i32,
    cake_eaten: u32,
    camera_should_focus: Vec3,
    camera_is_focus: Vec3,
}

// #[derive(Resource, Deref, DerefMut)]
// struct Random(ThreadRng);

const BOARD_SIZE_I: usize = 14;
const BOARD_SIZE_J: usize = 21;

const RESET_FOCUS: [f32; 3] = [BOARD_SIZE_I as f32 / 2., 0., BOARD_SIZE_J as f32 / 2.];

fn setup_cameras(mut commands: Commands, mut game: ResMut<Game>) {
    game.camera_should_focus = Vec3::from(RESET_FOCUS);
    game.camera_is_focus = game.camera_should_focus;
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(
            -(BOARD_SIZE_I as f32 / 2.),
            2. * BOARD_SIZE_J as f32 / 3.,
            BOARD_SIZE_J as f32 / 2. - 0.5,
        )
        .looking_at(game.camera_is_focus, Vec3::Y),
        ..Default::default()
    });
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut game: ResMut<Game>) {
    let mut rng = thread_rng();

    game.cake_eaten = 0;
    game.score = 0;
    game.player.i = BOARD_SIZE_I / 2;
    game.player.j = BOARD_SIZE_J / 2;
    game.player.move_cooldown = Timer::from_seconds(0.3, TimerMode::Once);

    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 10., 4.),
        point_light: PointLight {
            intensity: 2_000_000.0,
            shadows_enabled: true,
            range: 30.,
            ..Default::default()
        },
        ..Default::default()
    });

    let cell_scene = asset_server.load("moduls/AlienCake/tile.glb#Scene0");
    game.board = (0..BOARD_SIZE_J)
        .map(|j| {
            (0..BOARD_SIZE_I)
                .map(|i| {
                    let height = rng.gen_range(-0.1..0.1);
                    let scene = commands
                        .spawn(SceneBundle {
                            transform: Transform::from_xyz(i as f32, height - 0.2, j as f32),
                            scene: cell_scene.clone(),
                            ..Default::default()
                        })
                        .id();
                    Cell {
                        height,
                        shake: (rng.gen_range(-0.1..0.), rng.gen_range(0.0..0.1)),
                        is_up: rng.gen_bool(0.5),
                        entity: scene,
                    }
                })
                .collect()
        })
        .collect();

    game.player.entity = Some(
        commands
            .spawn(SceneBundle {
                transform: Transform {
                    translation: Vec3::new(
                        game.player.i as f32,
                        game.board[game.player.j][game.player.i].height,
                        game.player.j as f32,
                    ),
                    rotation: Quat::from_rotation_y(-PI / 2.),
                    ..Default::default()
                },
                scene: asset_server.load("moduls/AlienCake/alien.glb#Scene0"),
                ..Default::default()
            })
            .id(),
    );

    game.bonus.handle = asset_server.load("moduls/AlienCake/cakeBirthday.glb#Scene0");

    commands.spawn(
        TextBundle::from_section(
            "Score:",
            TextStyle {
                font_size: 40.,
                color: Color::rgb(0.5, 0.5, 1.0),
                ..Default::default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.),
            left: Val::Px(5.),
            ..Default::default()
        }),
    );
}

// 카메라와 윈도우 제외 모든 엔티티 제거
fn teardown(mut commands: Commands, entities: Query<Entity, (Without<Camera>, Without<Window>)>) {
    for entity in &entities {
        commands.entity(entity).despawn();
    }
}

fn move_player(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game: ResMut<Game>,
    mut transforms: Query<&mut Transform>,
    time: Res<Time>,
) {
    if game.player.move_cooldown.tick(time.delta()).finished() {
        let mut moved = false;
        let mut rotation = 0.;

        if keyboard_input.pressed(KeyCode::ArrowUp) {
            if game.player.i < BOARD_SIZE_I - 1 {
                game.player.i += 1;
            }
            rotation = -PI / 2.;
            moved = true;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            if game.player.i > 0 {
                game.player.i -= 1;
            }
            rotation = PI / 2.;
            moved = true;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            if game.player.j < BOARD_SIZE_J - 1 {
                game.player.j += 1;
            }
            rotation = PI;
            moved = true;
        }
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            if game.player.j > 0 {
                game.player.j -= 1;
            }
            rotation = 0.;
            moved = true;
        }

        if moved {
            game.player.move_cooldown.reset();
            *transforms.get_mut(game.player.entity.unwrap()).unwrap() = Transform {
                translation: Vec3::new(
                    game.player.i as f32,
                    game.board[game.player.j][game.player.i].height,
                    game.player.j as f32,
                ),
                rotation: Quat::from_rotation_y(rotation),
                ..Default::default()
            };
        }
    }

    if let Some(entity) = game.bonus.entity {
        if game.player.i == game.bonus.i && game.player.j == game.bonus.j {
            game.score += 2;
            game.cake_eaten += 1;
            commands.entity(entity).despawn_recursive();
            game.bonus.entity = None;
        }
    }
}

fn player_pos_set(mut query: Query<&mut Transform>, game: ResMut<Game>) {
    if let Some(player) = game.player.entity {
        let y = query
            .get(game.board[game.player.j][game.player.i].entity)
            .unwrap()
            .translation
            .y;
        if let Ok(mut player_transform) = query.get_mut(player) {
            player_transform.translation =
                Vec3::new(game.player.i as f32, y + 0.2, game.player.j as f32);
        }
    }
}

fn tile_set(mut query: Query<&mut Transform>, mut game: ResMut<Game>, time: Res<Time>) {
    for tile_x in game.board.iter_mut() {
        for tile in tile_x.iter_mut() {
            if let Ok(mut t) = query.get_mut(tile.entity) {
                let y = t.translation.y;
                if tile.is_up {
                    if y > tile.shake.1 + tile.height - 0.2 {
                        tile.is_up = false;
                    } else {
                        t.translation.y += time.delta_seconds() / 2.;
                    }
                } else {
                    if y < tile.shake.0 + tile.height - 0.2 {
                        tile.is_up = true;
                    } else {
                        t.translation.y -= time.delta_seconds() / 2.;
                    }
                }
            }
        }
    }
}

fn focus_camera(
    time: Res<Time>,
    mut game: ResMut<Game>,
    mut transforms: ParamSet<(Query<&mut Transform, With<Camera3d>>, Query<&Transform>)>,
) {
    const SPEED: f32 = 2.;
    if let (Some(player_entity), Some(bonus_entity)) = (game.player.entity, game.bonus.entity) {
        let transform_query = transforms.p1();
        if let (Ok(player_transform), Ok(bonus_transform)) = (
            transform_query.get(player_entity),
            transform_query.get(bonus_entity),
        ) {
            game.camera_should_focus = player_transform
                .translation
                .lerp(bonus_transform.translation, 0.5);
        }
    } else if let Some(player_entity) = game.player.entity {
        if let Ok(player_transform) = transforms.p1().get(player_entity) {
            game.camera_should_focus = player_transform.translation;
        }
    } else {
        game.camera_should_focus = Vec3::from(RESET_FOCUS);
    }

    let mut camera_motion = game.camera_should_focus - game.camera_is_focus;
    if camera_motion.length() > 0.2 {
        camera_motion *= SPEED * time.delta_seconds();
        game.camera_is_focus += camera_motion;
    }
    for mut transform in transforms.p0().iter_mut() {
        *transform = transform.looking_at(game.camera_is_focus, Vec3::Y);
    }
}

fn spawn_bonus(
    time: Res<Time>,
    mut timer: ResMut<BonusSpawnTimer>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    mut game: ResMut<Game>,
) {
    let mut rng = thread_rng();

    if !timer.0.tick(time.delta()).finished() {
        return;
    }

    if let Some(entity) = game.bonus.entity {
        game.score -= 3;
        commands.entity(entity).despawn_recursive();
        game.bonus.entity = None;
        if game.score <= -5 {
            next_state.set(GameState::GameOver);
            return;
        }
    }

    loop {
        game.bonus.i = rng.gen_range(0..BOARD_SIZE_I);
        game.bonus.j = rng.gen_range(0..BOARD_SIZE_J);
        if game.bonus.i != game.player.i || game.bonus.j != game.player.j {
            break;
        }
    }
    game.bonus.entity = Some(
        commands
            .spawn(SceneBundle {
                transform: Transform::from_xyz(
                    game.bonus.i as f32,
                    game.board[game.bonus.j][game.bonus.i].height + 0.2,
                    game.bonus.j as f32,
                ),
                scene: game.bonus.handle.clone(),
                ..Default::default()
            })
            .with_children(|ch| {
                ch.spawn(PointLightBundle {
                    point_light: PointLight {
                        color: Color::rgb(1., 1., 0.),
                        intensity: 500_000.0,
                        range: 10.0,
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(0., 2., 0.),
                    ..Default::default()
                });
            })
            .id(),
    );
}

fn rotate_bonus(game: Res<Game>, time: Res<Time>, mut transforms: Query<&mut Transform>) {
    let y = transforms
        .get(game.board[game.bonus.j][game.bonus.i].entity)
        .unwrap()
        .translation
        .y;
    if let Some(entity) = game.bonus.entity {
        if let Ok(mut cake_transform) = transforms.get_mut(entity) {
            cake_transform.rotate_y(time.delta_seconds());
            cake_transform.scale =
                Vec3::splat(1. + (game.score as f32 / 10. * time.elapsed_seconds().sin()).abs());
            cake_transform.translation =
                Vec3::new(game.bonus.i as f32, y + 0.2, game.bonus.j as f32);
        }
    }
}

fn scoreboard_system(game: Res<Game>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    text.sections[0].value = format!("Sugar Rush: {}", game.score);
}

fn gameover_keyboard(
    mut next_state: ResMut<NextState<GameState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        next_state.set(GameState::Playing);
    }
}

fn display_score(mut commands: Commands, game: Res<Game>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|p| {
            p.spawn(TextBundle::from_section(
                format!("Cake eaten: {}", game.cake_eaten),
                TextStyle {
                    font_size: 80.,
                    color: Color::rgb(0.5, 0.5, 1.),
                    ..Default::default()
                },
            ));
        });
}
