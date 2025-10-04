use std::ops::DerefMut;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_ecs_tilemap::prelude::*;
use chip_8::Chip8Emulator;

#[derive(Message)]
pub struct LoadRomMessage(pub std::path::PathBuf);

#[derive(Resource)]
struct Emulator(Chip8Emulator);

#[derive(Resource, Eq, PartialEq)]
enum EmulatorState {
    Stop,
    Step,
    Run,
}
impl Default for EmulatorState {
    fn default() -> Self { Self::Stop }
}

pub fn chip8_emulator_plugin(app: &mut App) {
    let emu_resource = Emulator(Chip8Emulator::new(&[]));

    app
        .add_plugins(TilemapPlugin)
        .add_systems(Startup, setup)
        .add_message::<LoadRomMessage>()
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .init_resource::<EmulatorState>()
        .insert_resource(emu_resource)
        .add_systems(FixedUpdate, update_emulator.run_if(resource_equals(EmulatorState::Run)))
        .add_systems(Update, reload_emulator)
        ;
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
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
        bevy::render::render_resource::TextureDimension::D2,
        &[255, 255, 255, 255],
        bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb,
        bevy::asset::RenderAssetUsages::RENDER_WORLD,
    );
    let white_image_handle = asset_server.add(white_image);

    let black_image = Image::new_fill(
        bevy::render::render_resource::Extent3d { width: single_tile_size as u32, height: single_tile_size as u32, depth_or_array_layers: 1 },
        bevy::render::render_resource::TextureDimension::D2,
        &[0, 0, 0, 255],
        bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb,
        bevy::asset::RenderAssetUsages::RENDER_WORLD,
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
    mut state: ResMut<EmulatorState>,
    mut tile_query: Query<(&TilePos, &mut TileTextureIndex)>,
) {
    let tick_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| emulator.0.tick()));
    if let Err(e) = tick_result {
        eprintln!("{e:?}");
        println!("{}", emulator.0);
        *state.deref_mut() = EmulatorState::Stop;
        return
    }
    for (pos, mut texture) in tile_query.iter_mut() {
        let tile_pos = ((31 - pos.y) * 64 + pos.x) as usize;
        let d_pixel = emulator.0.display_ram[tile_pos];
        texture.0 = (d_pixel & 0x80) as u32;
    }
}

fn reload_emulator(
    mut rom_message: MessageReader<LoadRomMessage>,
    mut emulator: ResMut<Emulator>,
    mut state: ResMut<EmulatorState>,
) {
    for ev in rom_message.read() {
        let path = ev.0.clone();
        let path_result = std::panic::catch_unwind(|| {
            std::fs::read(path).unwrap()
        });

        let contents = match path_result {
            Err(e) => { eprintln!("Could not read selected file: {e:?}"); return },
            Ok(v) => v,
        };

        emulator.0 = Chip8Emulator::new(contents.as_slice());
        *state.deref_mut() = EmulatorState::Run;
    }
}
