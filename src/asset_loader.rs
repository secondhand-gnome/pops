use bevy::prelude::*;

pub struct AssetLoaderPlugin;

#[derive(Resource, Debug)]
pub struct TextureAssets {
    // Textures go here
    raw_kernel: Handle<Image>,
}

#[derive(Resource, Debug)]
pub struct TextureAtlasAssets {
    // Texture atlases go here
}

#[derive(Resource, Debug)]
pub struct FontAssets {
    pub default: Handle<Font>,
}

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, load_assets);
    }
}

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let png = |sprite_name: &str| -> Handle<Image> {
        asset_server.load(format!("sprites/{sprite_name}.png"))
    };

    commands.insert_resource(TextureAssets {
        raw_kernel: png("raw-kernel"),
    });

    commands.insert_resource(TextureAtlasAssets {
        // Texture atlases go here
    });
    commands.insert_resource(FontAssets {
        default: asset_server.load("fonts/noto-sans/NotoSansMono-Bold.ttf"),
    });
}
