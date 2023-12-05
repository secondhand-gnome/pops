use bevy::prelude::*;

pub struct AssetLoaderPlugin;

#[derive(Resource, Debug)]
pub struct TextureAssets {
    // Textures go here
    pub raw_kernel: Handle<Image>,
    pub skillet: Handle<Image>,
    pub popcorn_box: Handle<Image>,
    pub auto_kettle: Handle<Image>,
}

#[derive(Resource, Debug)]
pub struct TextureAtlasAssets {
    // Texture atlases go here
    pub kernel: Handle<TextureAtlas>,
    pub auto_kettle: Handle<TextureAtlas>,
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

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let png = |sprite_name: &str| -> Handle<Image> {
        asset_server.load(format!("sprites/{sprite_name}.png"))
    };

    commands.insert_resource(TextureAssets {
        // Textures go here
        raw_kernel: png("raw-kernel"),
        skillet: png("skillet"),
        popcorn_box: png("popcorn-box"),
        auto_kettle: png("auto-kettle"),
    });

    commands.insert_resource(TextureAtlasAssets {
        // Texture atlases go here
        kernel: texture_atlases.add(TextureAtlas::from_grid(
            png("kernel-sheet"),
            Vec2::new(16., 16.),
            2,
            1,
            None,
            None,
        )),
        auto_kettle: texture_atlases.add(TextureAtlas::from_grid(
            png("auto-kettle"),
            Vec2::new(16., 16.),
            4,
            1,
            None,
            None,
        )),
    });
    commands.insert_resource(FontAssets {
        default: asset_server.load("fonts/noto-sans/NotoSansMono-Bold.ttf"),
    });
}
