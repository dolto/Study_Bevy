use std::{fs::File, io::Write, time::Duration};

use bevy::{
    asset::StrongHandle,
    prelude::*,
    render::camera::{CameraMainTextureUsages, CameraRenderGraph, Exposure},
    tasks::IoTaskPool,
};
use rand::Rng;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .register_type::<MyComponent>()
        .register_type::<MyResource>()
        .add_systems(Startup, setup)
        .add_systems(Update, (scene_controll, cuboid_controller))
        .run();
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct MyComponent {
    pub value: String,
    #[reflect(skip_serializing)]
    pub _time: Duration,
}
impl FromWorld for MyComponent {
    fn from_world(world: &mut World) -> Self {
        let time = world.resource::<Time>();
        MyComponent {
            value: "none value".to_string(),
            _time: time.elapsed(),
        }
    }
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
struct MyResource {
    pub value: String,
}

const SCENE_FILE_PATH: &str = "scenes/load_scene_example.scn.ron";
fn setup(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 5., 10.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
fn scene_controll(
    mut commands: Commands,
    assets_server: Res<AssetServer>,
    keyboard: Res<ButtonInput<KeyCode>>,
    world: &World,
) {
    // Save Scene
    if keyboard.just_pressed(KeyCode::KeyS) {
        // let mut scene_world = World::new();
        // let type_registry = world.resource::<AppTypeRegistry>().clone();
        // scene_world.insert_resource(type_registry);

        // let scene = DynamicScene::from_world(&scene_world);

        // let scene = DynamicSceneBuilder::from_world(world)
        //     .deny_all()
        //     .deny_all_resources()
        //     .allow::<MyComponent>()
        //     .allow::<Transform>()
        //     .allow::<GlobalTransform>()
        //     .allow::<Visibility>()
        //     .allow::<InheritedVisibility>()
        //     .allow::<ViewVisibility>()
        //     .allow_resource::<MyResource>()
        //     .extract_entities(world.iter_entities().map(|entity| entity.id()))
        //     .extract_resources()
        //     .build();

        let scene = DynamicSceneBuilder::from_world(world)
            .deny_resource::<Time<Real>>()
            .deny::<CameraRenderGraph>()
            .deny::<Exposure>()
            .deny::<CameraMainTextureUsages>()
            .extract_entities(world.iter_entities().map(|entity| entity.id()))
            .extract_resources()
            .build();

        let type_registry = world.resource::<AppTypeRegistry>();
        let serialized_scene = scene.serialize_ron(type_registry).unwrap();

        IoTaskPool::get()
            .spawn(async move {
                File::create(format!("assets/{SCENE_FILE_PATH}"))
                    .and_then(|mut file| file.write(serialized_scene.as_bytes()))
                    .expect("Error while writing secen to file!!!");
            })
            .detach();
    }

    // Load Scene
    if keyboard.just_pressed(KeyCode::KeyL) {
        commands.spawn(DynamicSceneBundle {
            scene: assets_server.load(SCENE_FILE_PATH),
            ..default()
        });
    }
}
fn cuboid_controller(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    // Make Cuboid and MyComponent
    if keyboard.just_pressed(KeyCode::KeyN) {
        let mut random = rand::thread_rng();
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Cuboid::new(1., 1., 1.)),
                material: materials.add(Color::rgb_u8(
                    random.gen_range(0..255),
                    random.gen_range(0..255),
                    random.gen_range(0..255),
                )),
                transform: Transform::from_xyz(
                    random.gen_range(-4.0..4.),
                    random.gen_range(-4.0..4.),
                    random.gen_range(-4.0..4.),
                ),
                ..default()
            },
            MyComponent {
                value: "hello".to_string(),
                _time: Duration::default(),
            },
        ));
    }
}
