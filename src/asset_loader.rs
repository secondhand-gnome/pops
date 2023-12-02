use bevy::prelude::*;

pub struct AssetLoaderPlugin;

#[derive(Resource, Debug)]
pub struct TextureAssets {
    // Textures go here
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
    commands.insert_resource(TextureAssets {
        // Textures go here
    });

    commands.insert_resource(TextureAtlasAssets {
        // Texture atlases go here
    });
    commands.insert_resource(FontAssets {
        default: asset_server.load("fonts/noto-sans/NotoSansMono-Bold.ttf"),
    });
}
