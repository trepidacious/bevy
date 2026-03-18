//! Shows a tilemap chunk rendered with a single draw call, including different orientations of tiles (rotated, mirrored) and using different tileset indices, colors, alpha and visibility to show all tile features.

use bevy::{
    image::{ImageArrayLayout, ImageLoaderSettings},
    prelude::*,
    sprite_render::{AlphaMode2d, TileData, TileOrientation, TilemapChunk, TilemapChunkTileData},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(ClearColor(Color::srgb(0.5, 0.5, 0.9)))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    let chunk_size = uvec2(17, 1);
    let tile_display_size = UVec2::splat(32);

    let tile_data = (0..chunk_size.element_product())
        .map(|i| {
            let v = i as f32 / 16.0;
            Some(TileData {
                tileset_index: 0,
                color: Color::srgba(v, v, v, 1.0),
                visible: true,
                orientation: TileOrientation::Default,
            })
        })
        .collect();

    commands.spawn((
        TilemapChunk {
            chunk_size,
            tile_display_size,
            tileset: assets.load_with_settings(
                "textures/arrow.png",
                |settings: &mut ImageLoaderSettings| {
                    // The tileset texture is expected to be an array of tile textures, so we tell the
                    // `ImageLoader` that our texture is composed of 2 stacked tile images.
                    settings.array_layout = Some(ImageArrayLayout::RowCount { rows: 2 });
                },
            ),
            alpha_mode: AlphaMode2d::Blend,
        },
        TilemapChunkTileData(tile_data),
    ));

    commands.spawn(Camera2d);
}
