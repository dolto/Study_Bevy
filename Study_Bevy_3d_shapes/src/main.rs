use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
};
use std::f32::consts::PI;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(Rotate { value: Vec3::ZERO })
        .add_systems(Startup, setup)
        .add_systems(Update, rotate)
        .run();
}

#[derive(Component)]
struct Shape;
#[derive(Resource)]
struct Rotate {
    value: Vec3,
}

const X_EXTENT: f32 = 12.;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });

    let shapes = [
        meshes.add(Cuboid::default()),
        meshes.add(Capsule3d::default()),
        meshes.add(Torus::default()),
        meshes.add(Cylinder::default()),
        meshes.add(Sphere::default().mesh().ico(5).unwrap()),
        meshes.add(Sphere::default().mesh().uv(32, 18)),
    ];

    let num_shapes = shapes.len() as f32;

    for (i, shape) in shapes.into_iter().enumerate() {
        commands.spawn((
            PbrBundle {
                mesh: shape,
                material: debug_material.clone(),
                transform: Transform::from_xyz(
                    -X_EXTENT / 2. + i as f32 / (num_shapes - 1.) * X_EXTENT,
                    2.,
                    0.,
                )
                .with_rotation(Quat::from_rotation_x(-PI / 4.)),
                ..default()
            },
            Shape,
        ));
    }

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            intensity: 10000000.,
            range: 100.,
            ..default()
        },
        transform: Transform::from_xyz(8., 16., 8.),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(50., 50.)),
        material: materials.add(Color::SILVER),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 6., 12.).looking_at(Vec3::Y, Vec3::Y),
        ..default()
    });
}

fn rotate(
    mut query: Query<&mut Transform, With<Shape>>,
    time: Res<Time>,
    mut rotate: ResMut<Rotate>,
    keyborad: Res<ButtonInput<KeyCode>>,
) {
    if keyborad.pressed(KeyCode::Numpad2) {
        rotate.value.x += time.delta_seconds();
    } else if keyborad.pressed(KeyCode::Numpad8) {
        rotate.value.x -= time.delta_seconds();
    }
    if keyborad.pressed(KeyCode::Numpad6) {
        rotate.value.y += time.delta_seconds();
    } else if keyborad.pressed(KeyCode::Numpad4) {
        rotate.value.y -= time.delta_seconds();
    }
    for mut transform in &mut query {
        transform.rotate(Quat::from_euler(
            EulerRot::XYZ,
            rotate.value.x * time.delta_seconds(),
            rotate.value.y * time.delta_seconds(),
            rotate.value.z * time.delta_seconds(),
        ));
    }
}
fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;
    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];
    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];

    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    )
}
