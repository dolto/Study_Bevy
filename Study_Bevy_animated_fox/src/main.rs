use bevy::animation::{animate_targets, RepeatAnimation};
// build: cargo run --features bevy/dynamic_linking
use bevy::prelude::*;
use std::f32::consts::PI;
use std::time::Duration;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                setup_scene_onece_loaded.before(animate_targets),
                keyboard_animation_control,
            ),
        )
        .run();
}

#[derive(Resource)]
struct Animations {
    animations: Vec<AnimationNodeIndex>,
    #[allow(dead_code)]
    graph: Handle<AnimationGraph>,
}

const FOX_GLB: &str = "Fox.glb";
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    let mut graph = AnimationGraph::new();
    let animations = graph
        .add_clips(
            [
                GltfAssetLabel::Animation(2).from_asset(FOX_GLB),
                GltfAssetLabel::Animation(1).from_asset(FOX_GLB),
                GltfAssetLabel::Animation(0).from_asset(FOX_GLB),
            ]
            .into_iter()
            .map(|path| asset_server.load(path)),
            1.0,
            graph.root,
        )
        .collect();

    let graph = graphs.add(graph);
    commands.insert_resource(Animations {
        animations,
        graph: graph.clone(),
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(100., 100., 150.)
            .looking_at(Vec3::new(0.0, 20., 0.), Vec3::Y),
        ..Default::default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(50000., 50000.)),
        material: materials.add(Color::srgb(0.3, 0.5, 0.3)),
        ..Default::default()
    });

    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0., 1., -PI / 4.)),
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..Default::default()
        }
        .into(),
        ..Default::default()
    });

    commands.spawn(SceneBundle {
        scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset(FOX_GLB)),
        ..Default::default()
    });
}

fn setup_scene_onece_loaded(
    mut commands: Commands,
    animations: Res<Animations>,
    // Added<Struct> 형태의 문법은, 엔티티중, 해당 컴포넌트가 들어가고나서 한번만 실행되도록 함 즉, 새로운 컴포넌트가 들어왔을 경우에 실행하는 약간 이벤트스러운 쿼리? 인거같음
    mut players: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
) {
    for (entity, mut player) in &mut players {
        // println!("여우의 개수만큼만 실행되어야함");
        let mut transitions = AnimationTransitions::new();

        transitions
            .play(&mut player, animations.animations[0], Duration::ZERO)
            .repeat();

        commands
            .entity(entity)
            .insert(animations.graph.clone())
            .insert(transitions);
    }
}

fn keyboard_animation_control(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut animation_players: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
    animations: Res<Animations>,
    mut current_animation: Local<usize>,
) {
    for (mut player, mut transitions) in &mut animation_players {
        let Some((&playing_animation_index, _)) = player.playing_animations().next() else {
            continue;
        };

        if keyboard_input.just_pressed(KeyCode::Space) {
            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
            if playing_animation.is_paused() {
                playing_animation.resume();
            } else {
                playing_animation.pause();
            }
        }

        if keyboard_input.just_pressed(KeyCode::ArrowUp) {
            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
            let speed = playing_animation.speed();
            playing_animation.set_speed(speed * 1.2);
        } else if keyboard_input.just_pressed(KeyCode::ArrowDown) {
            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
            let speed = playing_animation.speed();
            playing_animation.set_speed(speed * 0.8);
        }

        if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
            let elapsed = playing_animation.seek_time();
            playing_animation.seek_to(elapsed - 0.1);
        } else if keyboard_input.just_pressed(KeyCode::ArrowRight) {
            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
            let elapsed = playing_animation.seek_time();
            playing_animation.seek_to(elapsed + 0.1);
        }

        if keyboard_input.just_pressed(KeyCode::Enter) {
            *current_animation = (*current_animation + 1) % animations.animations.len();
            transitions
                .play(
                    &mut player,
                    animations.animations[*current_animation],
                    Duration::from_millis(6000),
                )
                .repeat();
        }

        if keyboard_input.just_pressed(KeyCode::Digit1) {
            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
            playing_animation
                .set_repeat(RepeatAnimation::Count(1))
                .replay();
        } else if keyboard_input.just_pressed(KeyCode::Digit3) {
            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
            playing_animation
                .set_repeat(RepeatAnimation::Count(3))
                .replay();
        } else if keyboard_input.just_pressed(KeyCode::Digit5) {
            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
            playing_animation
                .set_repeat(RepeatAnimation::Count(5))
                .replay();
        } else if keyboard_input.just_pressed(KeyCode::KeyL) {
            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
            playing_animation
                .set_repeat(RepeatAnimation::Forever)
                .replay();
        }
    }
}
