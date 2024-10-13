use bevy::animation::animate_targets;
use bevy::color::palettes::css::{ANTIQUE_WHITE, DARK_GREEN, WHITE};
use bevy::prelude::*;
use bevy::ui::RelativeCursorPosition;

#[cfg(not(target_arch = "wasm32"))]
use std::{fs::File, path::Path};

use argh::FromArgs;
#[cfg(not(target_arch = "wasm32"))]
use bevy::asset::io::file::FileAssetReader;
#[cfg(not(target_arch = "wasm32"))]
use bevy::tasks::IoTaskPool;
#[cfg(not(target_arch = "wasm32"))]
use ron::ser::PrettyConfig;

static ANIMATION_GRAPH_PATH: &str = "Fox.animgraph.ron";
// 0: root, 1: bland 이라서 나머지 3개 애니메이션은 2, 3, 4,가 된다
static CLIP_NODE_INDICES: [u32; 3] = [2, 3, 4];
static HELP_TEXT: &str = "애니메이션 클립 노드를 클릭해서 가중치를 변경해보세요";
static NODE_TYPES: [NodeType; 5] = [
    NodeType::Clip(ClipNode::new("Idle", 0)),
    NodeType::Clip(ClipNode::new("Walk", 1)),
    NodeType::Blend("Root"),
    NodeType::Blend("Blend\n0.5"),
    NodeType::Clip(ClipNode::new("Run", 2)),
];

static NODE_RECTS: [NodeRect; 5] = [
    NodeRect::new(10.0, 10., 97.64, 48.41),
    NodeRect::new(10.0, 78.41, 97.64, 48.41),
    NodeRect::new(286.08, 78.41, 97.64, 48.41),
    NodeRect::new(148.04, 44.2, 97.64, 48.41),
    NodeRect::new(10.0, 146.8, 97.64, 48.41),
];

static HORIZONTAL_LINES: [Line; 6] = [
    Line::new(107.64, 34.21, 20.2),
    Line::new(107.64, 102.61, 20.2),
    Line::new(107.64, 171.02, 158.24),
    Line::new(127.84, 68.41, 20.2),
    Line::new(245.68, 68.41, 20.2),
    Line::new(265.88, 102.61, 20.2),
];

static VERTICAL_LINES: [Line; 2] = [
    Line::new(127.83, 34.21, 68.40),
    Line::new(265.88, 68.41, 102.61),
];

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    let args: Args = argh::from_env();
    #[cfg(target_arch = "wasm32")]
    let args = Args::from_args(&[], &[]).unwrap();

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Animation Graph Example".into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, (setup_assets, setup_scene, setup_ui))
        .add_systems(Update, init_animations.before(animate_targets))
        .add_systems(
            Update,
            (handle_weight_drag, update_ui, sync_weights).chain(),
        )
        .insert_resource(args)
        .insert_resource(AmbientLight {
            color: WHITE.into(),
            brightness: 100.,
        })
        .run();
}

#[derive(FromArgs, Resource)]
#[argh(description = "args")]
struct Args {
    #[argh(switch)]
    #[argh(description = "no_load")]
    no_load: bool,
    #[argh(switch)]
    #[argh(description = "save")]
    save: bool,
}

#[derive(Clone, Resource)]
struct ExampleAnimationGraph(Handle<AnimationGraph>);

#[derive(Component)]
struct ExampleAnimationWeights {
    weights: [f32; 3],
}
impl Default for ExampleAnimationWeights {
    fn default() -> Self {
        Self { weights: [1.; 3] }
    }
}
#[derive(Debug)]
struct NodeRect {
    left: f32,
    bottom: f32,
    width: f32,
    height: f32,
}
impl NodeRect {
    const fn new(left: f32, bottom: f32, width: f32, height: f32) -> NodeRect {
        NodeRect {
            left,
            bottom,
            width,
            height,
        }
    }
}

struct Line {
    left: f32,
    bottom: f32,
    length: f32,
}
impl Line {
    const fn new(left: f32, bottom: f32, length: f32) -> Self {
        Self {
            left,
            bottom,
            length,
        }
    }
}

enum NodeType {
    Clip(ClipNode),
    Blend(&'static str),
}

#[derive(Clone, Component)]
struct ClipNode {
    text: &'static str,
    index: usize,
}
impl ClipNode {
    const fn new(text: &'static str, index: usize) -> Self {
        Self { text, index }
    }
}

fn setup_assets_programmatically(
    commands: &mut Commands,
    asset_server: &mut AssetServer,
    animation_graphs: &mut Assets<AnimationGraph>,
    _save: bool,
) {
    let mut animation_graph = AnimationGraph::new();
    let blend_node = animation_graph.add_blend(0.5, animation_graph.root);
    animation_graph.add_clip(
        asset_server.load(GltfAssetLabel::Animation(0).from_asset("Fox.glb")),
        1.,
        blend_node,
    );
    animation_graph.add_clip(
        asset_server.load(GltfAssetLabel::Animation(1).from_asset("Fox.glb")),
        1.,
        animation_graph.root,
    );
    animation_graph.add_clip(
        asset_server.load(GltfAssetLabel::Animation(2).from_asset("Fox.glb")),
        1.,
        blend_node,
    );

    #[cfg(not(target_arch = "wasm32"))]
    if _save {
        let animation_graph = animation_graph.clone();
        IoTaskPool::get()
            .spawn(async move {
                let mut animation_graph_writer = File::create(Path::join(
                    &FileAssetReader::get_base_path(),
                    Path::join(Path::new("assets"), Path::new(ANIMATION_GRAPH_PATH)),
                ))
                .expect("애니메이션 그래프파일을 여는데 실패했습니다");
                ron::ser::to_writer_pretty(
                    &mut animation_graph_writer,
                    &animation_graph,
                    PrettyConfig::default(),
                )
                .expect("애니메이션 그래프를 직렬화하는데 실패했습니다.");
            })
            .detach();
    }

    let handle = animation_graphs.add(animation_graph);
    commands.insert_resource(ExampleAnimationGraph(handle));
}

fn setup_assets_via_serialized_animation_graph(
    commands: &mut Commands,
    asset_server: &mut AssetServer,
) {
    println!("이게 실행이 될거임");
    commands.insert_resource(ExampleAnimationGraph(
        asset_server.load(ANIMATION_GRAPH_PATH),
    ));
}
fn setup_assets(
    mut commands: Commands,
    mut asset_server: ResMut<AssetServer>,
    mut animation_graphs: ResMut<Assets<AnimationGraph>>,
    args: Res<Args>,
) {
    if args.no_load || args.save {
        setup_assets_programmatically(
            &mut commands,
            &mut asset_server,
            &mut animation_graphs,
            args.save,
        );
    } else {
        setup_assets_via_serialized_animation_graph(&mut commands, &mut asset_server);
    }
}

fn setup_help_text(commands: &mut Commands, asset_server: Res<AssetServer>) {
    let mut text_style = TextStyle::default();
    text_style.font = asset_server.load("MS Gothic.ttf");
    commands.spawn(TextBundle {
        text: Text::from_section(HELP_TEXT, text_style),
        style: Style {
            position_type: PositionType::Absolute,
            top: Val::Px(12.),
            left: Val::Px(12.),
            ..Default::default()
        },
        ..Default::default()
    });
}

fn setup_node_rects(commands: &mut Commands) {
    for (node_rect, node_type) in NODE_RECTS.iter().zip(NODE_TYPES.iter()) {
        let node_string = match *node_type {
            NodeType::Clip(ref clip) => clip.text,
            NodeType::Blend(text) => text,
        };

        let text = commands
            .spawn(TextBundle {
                text: Text::from_section(
                    node_string,
                    TextStyle {
                        font_size: 16.,
                        color: ANTIQUE_WHITE.into(),
                        ..Default::default()
                    },
                )
                .with_justify(JustifyText::Center),
                ..Default::default()
            })
            .id();

        let container = {
            let mut container = commands.spawn((
                NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        bottom: Val::Px(node_rect.bottom),
                        left: Val::Px(node_rect.left),
                        height: Val::Px(node_rect.height),
                        width: Val::Px(node_rect.width),
                        align_items: AlignItems::Center,
                        justify_items: JustifyItems::Center,
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    border_color: WHITE.into(),
                    ..Default::default()
                },
                Outline::new(Val::Px(1.), Val::ZERO, Color::WHITE),
            ));
            if let NodeType::Clip(ref clip) = node_type {
                container.insert((
                    Interaction::None,
                    RelativeCursorPosition::default(),
                    (*clip).clone(),
                ));
            }
            container.id()
        };

        if let NodeType::Clip(_) = node_type {
            // let background = commands
            //     .spawn(NodeBundle {
            //         style: Style {
            //             position_type: PositionType::Absolute,
            //             top: Val::Px(0.),
            //             left: Val::Px(0.),
            //             height: Val::Px(node_rect.height),
            //             width: Val::Px(node_rect.width),
            //             ..Default::default()
            //         },
            //         background_color: DARK_GREEN.into(),
            //         ..Default::default()
            //     })
            //     .id();
            // commands.entity(container).add_child(background);

            commands.entity(container).with_children(|p| {
                p.spawn(NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        top: Val::Px(0.),
                        left: Val::Px(0.),
                        height: Val::Px(node_rect.height),
                        width: Val::Px(node_rect.width),
                        ..Default::default()
                    },
                    background_color: DARK_GREEN.into(),
                    ..Default::default()
                });
            });
        }

        commands.entity(container).add_child(text);
    }
}

fn setup_node_lines(commands: &mut Commands) {
    for line in &HORIZONTAL_LINES {
        commands.spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(line.bottom),
                left: Val::Px(line.left),
                height: Val::Px(0.),
                width: Val::Px(line.length),
                border: UiRect::bottom(Val::Px(1.)),
                ..Default::default()
            },
            border_color: WHITE.into(),
            ..Default::default()
        });
    }

    for line in &VERTICAL_LINES {
        commands.spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(line.bottom),
                left: Val::Px(line.left),
                height: Val::Px(line.length),
                width: Val::Px(0.),
                border: UiRect::left(Val::Px(1.)),
                ..Default::default()
            },
            border_color: WHITE.into(),
            ..Default::default()
        });
    }
}
fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    setup_help_text(&mut commands, asset_server);
    setup_node_lines(&mut commands);
    setup_node_rects(&mut commands);
}

fn setup_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-10., 5., 13.).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
        ..Default::default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 10000000.,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(-4., 8., 13.),
        ..Default::default()
    });

    commands.spawn(SceneBundle {
        scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("Fox.glb")),
        transform: Transform::from_scale(Vec3::splat(0.07)),
        ..Default::default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Circle::new(7.)),
        material: materials.add(Color::srgb(0.3, 0.5, 0.3)),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..Default::default()
    });
}

fn init_animations(
    mut commands: Commands,
    mut query: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
    animation_graph: Res<ExampleAnimationGraph>,
) {
    for (entity, mut player) in &mut query {
        println!("한번만 실행");
        commands.entity(entity).insert((
            animation_graph.0.clone(),
            ExampleAnimationWeights::default(),
        ));
        for &node_index in &CLIP_NODE_INDICES {
            player.play(node_index.into()).repeat();
        }
    }
}

fn handle_weight_drag(
    mut interaction_query: Query<(&Interaction, &RelativeCursorPosition, &ClipNode)>,
    mut animation_weights_query: Query<&mut ExampleAnimationWeights>,
) {
    for (interaction, relative_cursor, clip_node) in &mut interaction_query {
        if matches!(*interaction, Interaction::Pressed) {
            // 상대적으로 클릭된 노드의 어느부분에 있는지 정규화
            let Some(pos) = relative_cursor.normalized else {
                continue;
            };
            for mut animation_weights in &mut animation_weights_query {
                animation_weights.weights[clip_node.index] = pos.x.clamp(0., 1.);
            }
        }
    }
}

fn update_ui(
    mut text_query: Query<&mut Text>,
    mut background_query: Query<&mut Style, Without<Text>>,
    container_query: Query<(&Children, &ClipNode)>,
    animation_weights_query: Query<&ExampleAnimationWeights, Changed<ExampleAnimationWeights>>,
) {
    for animation_weights in &animation_weights_query {
        for (children, clip_node) in &container_query {
            let mut bg_iter = background_query.iter_many_mut(children);
            if let Some(mut style) = bg_iter.fetch_next() {
                style.width =
                    Val::Px(NODE_RECTS[0].width * animation_weights.weights[clip_node.index]);
            }

            let mut text_iter = text_query.iter_many_mut(children);
            if let Some(mut text) = text_iter.fetch_next() {
                text.sections[0].value = format!(
                    "{}\n{:.2}",
                    clip_node.text, animation_weights.weights[clip_node.index]
                );
            }
        }
    }
}

fn sync_weights(mut query: Query<(&mut AnimationPlayer, &ExampleAnimationWeights)>) {
    for (mut animation_player, animation_weights) in &mut query {
        for (&animation_node_index, &animation_weight) in CLIP_NODE_INDICES
            .iter()
            .zip(animation_weights.weights.iter())
        {
            if !animation_player.animation_is_playing(animation_node_index.into()) {
                animation_player.play(animation_node_index.into());
            }

            if let Some(active_animation) =
                animation_player.animation_mut(animation_node_index.into())
            {
                active_animation.set_weight(animation_weight);
            }
        }
    }
}
