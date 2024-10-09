// build: cargo run --features bevy/dynamic_linking
use bevy::{prelude::*, reflect::TypePath, render::render_resource::AsBindGroup};
const SHADER_ASSET_PATH: &str = "animate_shader.wgsl";
fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MaterialPlugin::<CustomMaterial>::default()))
        .add_systems(Startup, setup)
        .run();
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct CustomMaterial {}
impl Material for CustomMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        SHADER_ASSET_PATH.into()
    }
}
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Cuboid::default()),
        transform: Transform::from_xyz(0.0, 0.5, 0.),
        material: materials.add(CustomMaterial {}),
        ..default()
    });

    commands.spawn_empty().insert(Camera3dBundle {
        transform: Transform::from_xyz(-2., 2., 5.).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}
