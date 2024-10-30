use bevy::{asset::LoadedFolder, prelude::*};
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    meshes: Res<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 기본적으로 에셋폴더는 애플리케이션의 root폴더의 assets폴더안에 존재함
    // 이는 CARGO_MANIFEST_DIR환경변수를 설정함으로서 변경할 수 있음
    // 애플리케이션을 cargo로 실행하면 해당 환경변수는 자동적으로 crate폴더가 됨(workspace)
    // 근데 왜 나는...

    let cube_handle = asset_server.load(
        GltfAssetLabel::Primitive {
            mesh: 0,
            primitive: 0,
        }
        .from_asset("models/cube/cube.gltf"),
    );
    let sphere_handle = asset_server.load(
        GltfAssetLabel::Primitive {
            mesh: 0,
            primitive: 0,
        }
        .from_asset("models/sphere/sphere.gltf"),
    );

    if let Some(sphere) = meshes.get(&sphere_handle) {
        info!("{:?}", sphere.primitive_topology());
    } else {
        info!("sphere hasn't loaded yet");
    }

    // 물론 다음과같이 폴더 전체를 가져오는 방법도 있음... (시간이 걸리는 작업)
    let _loaded_folder: Handle<LoadedFolder> = asset_server.load_folder("models/torus");

    let torus_handle = asset_server.load(
        GltfAssetLabel::Primitive {
            mesh: 0,
            primitive: 0,
        }
        .from_asset("models/torus/torus.gltf"),
    );

    let material_handle = materials.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.7, 0.6),
        ..Default::default()
    });

    commands.spawn(PbrBundle {
        mesh: torus_handle,
        material: material_handle.clone(),
        transform: Transform::from_xyz(-3., 0., 0.),
        ..Default::default()
    });
    commands.spawn(PbrBundle {
        mesh: cube_handle,
        material: material_handle.clone(),
        transform: Transform::from_xyz(0., 0., 0.),
        ..Default::default()
    });
    commands.spawn(PbrBundle {
        mesh: sphere_handle,
        material: material_handle.clone(),
        transform: Transform::from_xyz(3., 0., 0.),
        ..Default::default()
    });

    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4., 5., 4.),
        ..Default::default()
    });
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 3., 10.).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}
