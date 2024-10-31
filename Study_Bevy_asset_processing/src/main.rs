use bevy::{
    asset::{AssetLoader, AsyncReadExt},
    prelude::*,
};
use serde::{Deserialize, Serialize};
#[derive(Asset, TypePath, Debug)]
struct Text(String);
#[derive(Resource)]
struct TextAssets {
    a: Handle<Text>,
    b: Handle<Text>,
    c: Handle<Text>,
    d: Handle<Text>,
}

#[derive(Default)]
struct TextLoader;

#[derive(Clone, Default, Serialize, Deserialize)]
struct TextSettings {
    text_override: Option<String>,
}
impl AssetLoader for TextLoader {
    type Asset = Text;
    type Settings = ();
    type Error = std::io::Error;
    async fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader<'_>,
        _settings: &'a Self::Settings,
        _load_context: &'a mut bevy::asset::LoadContext<'_>,
    ) -> Result<Self::Asset, std::io::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let value = String::from_utf8(bytes).unwrap();
        // println!("{value}");
        Ok(Text(value))
    }

    fn extensions(&self) -> &[&str] {
        &["cool.ron"]
    }
}
fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(AssetPlugin {
            mode: AssetMode::Processed,
            ..Default::default()
        }),))
        .init_asset::<Text>()
        .init_asset_loader::<TextLoader>()
        .add_systems(Startup, setup)
        .add_systems(Update, print_text)
        .run();
}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(TextAssets {
        a: assets.load("a.cool.ron"),
        b: assets.load("foo/b.cool.ron"),
        c: assets.load("foo/c.cool.ron"),
        d: assets.load("d.cool.ron"),
    });
}

fn print_text(
    handles: Res<TextAssets>,
    texts: Res<Assets<Text>>,
    mut asset_events: EventReader<AssetEvent<Text>>,
) {
    if !asset_events.is_empty() {
        println!("Current Values:");
        println!("  a: {:?}", texts.get(&handles.a));
        println!("  b: {:?}", texts.get(&handles.b));
        println!("  c: {:?}", texts.get(&handles.c));
        println!("  d: {:?}", texts.get(&handles.d));
        println!("(You can modify source assets and their .meta files to hot-reload changes!)");
        println!();
        asset_events.clear();
    }
}
