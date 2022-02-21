#![allow(unused, dead_code)]
#![forbid(unsafe_code)]

use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

mod hardware;
mod bevy_interface;

const SCALE_FACTOR : u32 = 20;
const WINDOW_WIDTH : f32 = ((hardware::VRAM_WIDTH as u32) * SCALE_FACTOR) as f32;
const WINDOW_HEIGHT: f32 = ((hardware::VRAM_HEIGHT as u32) * SCALE_FACTOR) as f32;

fn main() {
    let mut cpu = hardware::CPU::new();

    let mut card = File::open("./roms/IBM Logo.ch8").expect("file not found");
    let mut buffer = [0u8; 3584];
    let bytes_read = if let Ok(bytes_read) = card.read(&mut buffer) {
        bytes_read
    } else {
        0
    };
    cpu.load_card(&buffer);

    App::new()
        .insert_resource(WindowDescriptor {
            title: "Chip-8 Emulator".to_string(),
            width : WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            ..Default::default()
        })
        .insert_resource(cpu)
        .add_plugins(DefaultPlugins)
        .add_plugin(TilemapPlugin)
        .add_startup_system(setup)
        .add_system(bevy_interface::helpers::set_texture_filters_to_nearest)
        .add_system(load_display)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.5))
                .with_system(tick_cpu)
        )
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut map_query: MapQuery) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let texture_handle = asset_server.load("tiles.png");

    let map_entity = commands.spawn().id();
    let mut map = Map::new(0u16, map_entity);

    let (mut layer_builder, _) = LayerBuilder::new(
        &mut commands,
        LayerSettings::new(
            MapSize(8, 4),
            ChunkSize(8, 8),
            TileSize(16.0, 16.0),
            TextureSize(96.0, 16.0),
        ),
        0u16,
        0u16,
    );

    layer_builder.set_all(TileBundle::default());
    let layer_entity = map_query.build_layer(&mut commands, layer_builder, texture_handle);
    map.add_layer(&mut commands, 0u16, layer_entity);

    commands
        .entity(map_entity)
        .insert(map)
        .insert(Transform::from_xyz(-(WINDOW_WIDTH as f32 / 2.5), -(WINDOW_HEIGHT as f32 / 2.5), 0.0))
        .insert(GlobalTransform::default());
}

fn load_display(cpu: Res<hardware::CPU>, mut query: Query<(&mut Tile, &TileParent, &TilePos)>, mut chunk_query: Query<&mut Chunk>) {
    let mut chunks = HashSet::new();

    for (index, (mut tile, tile_parent, tile_pos)) in query.iter_mut().enumerate() {
        let vram_get = cpu.vram_tile(tile_pos.0, tile_pos.1);
        let index = if vram_get == 0 {
            3
        } else {
            0
        };
        tile.texture_index = index;
        chunks.insert(tile_parent.chunk);
    }

    for chunk_entity in chunks.drain() {
        if let Ok(mut chunk) = chunk_query.get_mut(chunk_entity) {
            chunk.needs_remesh = true;
        }
    }
}

fn tick_cpu(cpu: ResMut<hardware::CPU>) {
    cpu.into_inner().cycle();
}