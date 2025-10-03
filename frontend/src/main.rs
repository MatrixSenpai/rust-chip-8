use bevy::{asset::RenderAssetUsages, prelude::*, render::render_resource::{TextureDimension, TextureFormat}, window::PrimaryWindow};
use bevy_ecs_tilemap::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use chip_8::Chip8Emulator;

#[derive(Resource)]
struct Emulator(Chip8Emulator);

#[derive(Resource, Eq, PartialEq)]
struct EmulatorPaused(bool);

fn main() {
    let rom = std::fs::read("./chip-8/roms/BC_test.ch8").unwrap();
    let emu = Chip8Emulator::new(rom.as_slice());
    let emu_resource = Emulator(emu);

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TilemapPlugin)
        .add_plugins(EguiPlugin::default())
        .add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new())
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .insert_resource(emu_resource)
        .insert_resource(EmulatorPaused(false))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, update_emulator.run_if(resource_equals(EmulatorPaused(false))))
        .run();
}

fn setup(
    mut commands: Commands,
    window: Single<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2d);

    let map_size = TilemapSize { x: 64, y: 32 };

    let tilemap_entity = commands.spawn_empty().id();
    let mut tilemap_storage = TileStorage::empty(map_size);

    bevy_ecs_tilemap::helpers::filling::fill_tilemap(
        TileTextureIndex(0), 
        map_size, 
        TilemapId(tilemap_entity), 
        &mut commands, 
        &mut tilemap_storage
    );

    let single_tile_size = std::cmp::min(
        (window.width() as u32 - 10) / 64,
        (window.height() as u32 - 10) / 32,
    ) as f32;
    let tile_size = TilemapTileSize::new(single_tile_size, single_tile_size);
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    let white_image = Image::new_fill(
        bevy::render::render_resource::Extent3d { width: single_tile_size as u32, height: single_tile_size as u32, depth_or_array_layers: 1 },
        TextureDimension::D2,
        &[255, 255, 255, 255],
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    );
    let white_image_handle = asset_server.add(white_image);

    let black_image = Image::new_fill(
        bevy::render::render_resource::Extent3d { width: single_tile_size as u32, height: single_tile_size as u32, depth_or_array_layers: 1 },
        TextureDimension::D2,
        &[0, 0, 0, 255],
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    );
    let black_image_handle = asset_server.add(black_image);

    let texture = TilemapTexture::Vector(vec![black_image_handle, white_image_handle]);

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size, map_type, texture, tile_size,
        size: map_size,
        storage: tilemap_storage,
        anchor: TilemapAnchor::Center,
        ..default()
    });
}

fn update_emulator(
    mut emulator: ResMut<Emulator>,
    mut flag: ResMut<EmulatorPaused>,
    mut tile_query: Query<(&TilePos, &mut TileTextureIndex)>,
) {
    let tick_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| emulator.0.tick()));
    if let Err(e) = tick_result {
        eprintln!("{e:?}");
        println!("{}", emulator.0);
        flag.0 = true;
        return
    }
    for (pos, mut texture) in tile_query.iter_mut() {
        let tile_pos = ((31 - pos.y) * 64 + pos.x) as usize;
        let d_pixel = emulator.0.display_ram[tile_pos];
        texture.0 = (d_pixel & 0x80) as u32;
    }
}
