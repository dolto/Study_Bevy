use std::{io::Read, marker::PhantomData};

use bevy::{
    asset::{io::VecReader, AssetLoader, AsyncReadExt, ErasedLoadedAsset, LoadDirectError},
    prelude::*,
};
use flate2::read::GzDecoder;
use thiserror::Error;

#[derive(Asset, TypePath)]
struct GzAsset {
    uncompressed: ErasedLoadedAsset,
}

#[derive(Default)]
struct GzAssetLoader;
#[non_exhaustive]
#[derive(Debug, Error)]
enum GzAssetLoaderError {
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    #[error("Could not determine file path of uncompressed asset")]
    IndeterminateFilePath,
    #[error("Could not load contained asset: {0}")]
    LoadDirectError(#[from] LoadDirectError),
}

impl AssetLoader for GzAssetLoader {
    type Asset = GzAsset;
    type Settings = ();
    type Error = GzAssetLoaderError;
    async fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader<'_>,
        _settings: &'a Self::Settings,
        load_context: &'a mut bevy::asset::LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let compressed_path = load_context.path();
        let file_name = compressed_path
            .file_name()
            .ok_or(GzAssetLoaderError::IndeterminateFilePath)?
            .to_string_lossy();
        let uncompressed_file_name = file_name
            .strip_suffix(".gz")
            .ok_or(GzAssetLoaderError::IndeterminateFilePath)?;
        let contained_path = compressed_path.join(uncompressed_file_name);

        let mut bytes_compressed = Vec::new();
        reader.read_to_end(&mut bytes_compressed).await?;
        let mut decoder = GzDecoder::new(bytes_compressed.as_slice());
        let mut bytes_uncompressed = Vec::new();
        decoder.read_to_end(&mut bytes_uncompressed)?;
        let mut reader = VecReader::new(bytes_uncompressed);
        let uncompressed = load_context
            .loader()
            .direct()
            .with_reader(&mut reader)
            .untyped()
            .load(contained_path)
            .await?;

        Ok(GzAsset { uncompressed })
    }

    fn extensions(&self) -> &[&str] {
        &["gz"]
    }
}
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_asset::<GzAsset>()
        .init_asset_loader::<GzAssetLoader>()
        .add_systems(Startup, setup)
        .add_systems(Update, decompress::<Image>)
        .run();
}

#[derive(Component, Default)]
struct Compressed<T> {
    compressed: Handle<GzAsset>,
    _phantom: PhantomData<T>,
}
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        Compressed::<Image> {
            compressed: asset_server.load("data/compressed_image.png.gz"),
            ..Default::default()
        },
        Sprite::default(),
        TransformBundle::default(),
        VisibilityBundle::default(),
    ));
}

fn decompress<A: Asset>(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut compressed_assets: ResMut<Assets<GzAsset>>,
    query: Query<(Entity, &Compressed<A>)>,
) {
    for (entity, Compressed { compressed, .. }) in query.iter() {
        let Some(GzAsset { uncompressed }) = compressed_assets.remove(compressed) else {
            continue;
        };

        let uncompressed = uncompressed.take::<A>().unwrap();
        commands
            .entity(entity)
            .remove::<Compressed<A>>()
            .insert(asset_server.add(uncompressed));
    }
}
