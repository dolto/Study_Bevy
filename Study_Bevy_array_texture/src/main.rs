use std::time::Duration;

use bevy::{asset::LoadState, prelude::*, render::render_resource::AsBindGroup};
use rand::{rngs::ThreadRng, Rng};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct ArrayTextureMaterial {
    #[texture(0, dimension = "2d_array")]
    #[sampler(1)]
    array_texture: Handle<Image>,
}
impl Material for ArrayTextureMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "shaders/array_texture.wgsl".into()
    }
}
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            MaterialPlugin::<ArrayTextureMaterial>::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (create_array_texture, move_cuboid))
        .run();
}

#[derive(Resource)]
struct LoadingTexture {
    is_loaded: bool,
    handle: Handle<Image>,
}
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(LoadingTexture {
        is_loaded: false,
        handle: asset_server.load("textures/array_texture.png"),
    });

    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(3., 2., 1.).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(5., 5., 5.).looking_at(Vec3::new(1.5, 0., 0.), Vec3::Y),
        ..default()
    });
}

fn create_array_texture(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut loading_texture: ResMut<LoadingTexture>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ArrayTextureMaterial>>,
) {
    if loading_texture.is_loaded
        || asset_server.load_state(loading_texture.handle.id()) != LoadState::Loaded
    {
        return;
    }
    loading_texture.is_loaded = true;
    let image = images.get_mut(&loading_texture.handle).unwrap();

    let array_layers = 4;
    // 세로로 4칸을 동일한 간격으로 잘라라
    image.reinterpret_stacked_2d_as_array(array_layers);

    let mesh_handle = meshes.add(Cuboid::default());
    let material_handle = materials.add(ArrayTextureMaterial {
        array_texture: loading_texture.handle.clone(),
    });
    for x in 0..4 {
        commands.spawn(MaterialMeshBundle {
            mesh: mesh_handle.clone(),
            material: material_handle.clone(),
            transform: Transform::from_xyz(x as f32 + 0.5, 0., 0.),
            ..Default::default()
        });
    }
}

fn move_cuboid(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Handle<Mesh>>>,
    time: Res<Time>,
) {
    let mut direction = Vec3::ZERO;
    if input.pressed(KeyCode::KeyW) {
        direction += Vec3::Z;
    }
    if input.pressed(KeyCode::KeyS) {
        direction -= Vec3::Z;
    }

    if input.pressed(KeyCode::KeyA) {
        direction += Vec3::X;
    }
    if input.pressed(KeyCode::KeyD) {
        direction -= Vec3::X;
    }

    if direction.length() >= 0.5 {
        direction = direction.normalize() * time.delta_seconds();
    }

    for mut cube in &mut query {
        cube.translation += direction;
    }
}

struct GrassGroun(Timer);
impl GrassGroun {
    pub fn new(rng: &mut ThreadRng) -> Self {
        Self(Timer::new(
            Duration::from_secs(rng.gen_range(3..10)),
            TimerMode::Repeating,
        ))
    }
}
