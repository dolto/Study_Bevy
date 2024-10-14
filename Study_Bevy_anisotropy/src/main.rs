use bevy::{color::palettes::css::WHITE, core_pipeline::Skybox, prelude::*, time::Stopwatch};
fn main() {
    App::new()
        .init_resource::<AppStatus>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Anisotropy Example".into(),
                ..default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (create_material_variants, animate_light, rotate_camera),
        )
        .add_systems(Update, (handle_input, update_help_text).chain())
        .run();
}

const CAMERA_INITIAL_POSITION: Vec3 = Vec3::new(-0.4, 0., 0.);

#[derive(Clone, Copy, PartialEq, Default)]
enum LightMode {
    #[default]
    Directional,
    Point,
    EnvironmentMap,
}

#[derive(Resource)]
struct AppStatus {
    light_mode: LightMode,
    anisotropy_enabled: bool,
}
impl Default for AppStatus {
    fn default() -> Self {
        Self {
            light_mode: default(),
            anisotropy_enabled: true,
        }
    }
}
impl AppStatus {
    fn create_help_text(&self, asset_server: &AssetServer) -> Text {
        let material_variant_help_text = if self.anisotropy_enabled {
            "Press Enter to disable anisotropy"
        } else {
            "Press Enter to enable anisotropy"
        };

        let light_help_text = match self.light_mode {
            LightMode::Directional => "Press Space to switch to a point light",
            LightMode::Point => "Press Space to switch to an environment map",
            LightMode::EnvironmentMap => "Press Space to switch to a directional light",
        };

        Text::from_section(
            format!("{}\n{}", material_variant_help_text, light_help_text),
            TextStyle {
                font_size: 24.,
                ..Default::default()
            },
        )
    }
}

const ANISOTROPY_BARN_LAMP_PATH: &str = "AnisotropyBarnLamp.gltf";
fn setup(mut commands: Commands, asset_server: Res<AssetServer>, app_status: Res<AppStatus>) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(CAMERA_INITIAL_POSITION)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    spawn_directional_light(&mut commands);

    commands.spawn(SceneBundle {
        scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset(ANISOTROPY_BARN_LAMP_PATH)),
        transform: Transform::from_xyz(0., 0.07, -0.13),
        ..Default::default()
    });

    spawn_text(&mut commands, &asset_server, &app_status);
}

fn spawn_directional_light(commands: &mut Commands) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: WHITE.into(),
            illuminance: 3000.,
            ..Default::default()
        },
        ..Default::default()
    });
}

fn spawn_text(commands: &mut Commands, asset_server: &AssetServer, app_status: &AppStatus) {
    commands.spawn(
        TextBundle {
            text: app_status.create_help_text(asset_server),
            ..Default::default()
        }
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.),
            left: Val::Px(10.),
            ..Default::default()
        }),
    );
}

#[derive(Component)]
struct MaterialVariants {
    anisotropic: Handle<StandardMaterial>,
    isotropic: Handle<StandardMaterial>,
}
fn create_material_variants(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    new_meshes: Query<
        (Entity, &Handle<StandardMaterial>),
        (Added<Handle<StandardMaterial>>, Without<MaterialVariants>),
    >,
) {
    for (entity, anisotropic_material_handle) in new_meshes.iter() {
        let Some(anisotropic_material) = materials.get(anisotropic_material_handle).cloned() else {
            continue;
        };

        commands.entity(entity).insert(MaterialVariants {
            anisotropic: anisotropic_material_handle.clone(),
            isotropic: materials.add(StandardMaterial {
                anisotropy_texture: None,
                anisotropy_strength: 0.,
                anisotropy_rotation: 0.,
                ..anisotropic_material
            }),
        });
    }
}

fn animate_light(
    mut lights: Query<&mut Transform, Or<(With<DirectionalLight>, With<PointLight>)>>,
    time: Res<Time>,
) {
    let now = time.elapsed_seconds();
    for mut transform in lights.iter_mut() {
        transform.translation =
            Vec3::new(f32::cos(now), 1.0, f32::sin(now)) * Vec3::new(3., 4., 3.);
    }
}

fn rotate_camera(
    mut camera: Query<&mut Transform, With<Camera>>,
    app_status: Res<AppStatus>,
    time: Res<Time>,
    mut stopwatch: Local<Stopwatch>,
) {
    if app_status.light_mode == LightMode::EnvironmentMap {
        stopwatch.tick(time.delta());
    }

    let now = stopwatch.elapsed_secs();
    for mut transform in &mut camera {
        *transform = Transform::from_translation(
            Quat::from_rotation_y(now).mul_vec3(CAMERA_INITIAL_POSITION),
        )
        .looking_at(Vec3::ZERO, Vec3::Y)
    }
}

fn handle_input(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    cameras: Query<Entity, With<Camera>>,
    lights: Query<Entity, Or<(With<DirectionalLight>, With<PointLight>)>>,
    mut meshes: Query<(&mut Handle<StandardMaterial>, &MaterialVariants)>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut app_status: ResMut<AppStatus>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        match app_status.light_mode {
            LightMode::Directional => {
                app_status.light_mode = LightMode::Point;
                for light in &lights {
                    commands.entity(light).despawn();
                }
                spawn_point_light(&mut commands);
            }

            LightMode::Point => {
                app_status.light_mode = LightMode::EnvironmentMap;
                for light in &lights {
                    commands.entity(light).despawn();
                }
                for camera in &cameras {
                    add_skybox_and_environment_map(&mut commands, &asset_server, camera);
                }
            }

            LightMode::EnvironmentMap => {
                app_status.light_mode = LightMode::Directional;
                for camera in &cameras {
                    commands
                        .entity(camera)
                        .remove::<Skybox>()
                        .remove::<EnvironmentMapLight>();
                }
                spawn_directional_light(&mut commands);
            }
        }
    }

    if keyboard.just_pressed(KeyCode::Enter) {
        app_status.anisotropy_enabled = !app_status.anisotropy_enabled;

        for (mut material_handle, material_variants) in &mut meshes {
            *material_handle = if app_status.anisotropy_enabled {
                material_variants.anisotropic.clone()
            } else {
                material_variants.isotropic.clone()
            }
        }
    }
}

fn spawn_point_light(commands: &mut Commands) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            color: WHITE.into(),
            intensity: 200000.,
            ..Default::default()
        },
        ..Default::default()
    });
}

fn add_skybox_and_environment_map(
    commands: &mut Commands,
    asset_server: &AssetServer,
    entity: Entity,
) {
    commands
        .entity(entity)
        .insert(Skybox {
            brightness: 5000.,
            image: asset_server.load("pisa_specular_rgb9e5_zstd.ktx2"),
        })
        .insert(EnvironmentMapLight {
            diffuse_map: asset_server.load("pisa_diffuse_rgb9e5_zstd.ktx2"),
            specular_map: asset_server.load("pisa_specular_rgb9e5_zstd.ktx2"),
            intensity: 2500.0,
        });
}

fn update_help_text(
    mut text_query: Query<&mut Text>,
    app_status: Res<AppStatus>,
    asset_server: Res<AssetServer>,
) {
    for mut text in &mut text_query {
        *text = app_status.create_help_text(&asset_server);
    }
}
