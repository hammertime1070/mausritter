use bevy::prelude::*;

use crate::board::components::Position;

pub const TILE_SIZE: f32 = 32.;
pub const TILE_Z: f32 = 0.;
pub const PIECE_Z: f32 = 10.;
pub const PIECE_SPEED: f32 = 10.;
pub const POSITION_TOLERANCE: f32 = 0.1;

mod assets;
mod pieces;
mod tiles;

#[derive(Resource)]
pub struct GraphicsAssets {
    pub sprite_texture: Handle<TextureAtlas>,
}

#[derive(Event)]
pub struct GraphicsWaitEvent;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GraphicsWaitEvent>()
            .add_systems(Startup, assets::load_assets)
            .add_systems(Update, tiles::spawn_tile_renderer)
            .add_systems(Update, pieces::spawn_piece_renderer)
            .add_systems(Update, pieces::update_piece_position);
    }
}

fn get_world_position(position: &Position, z: f32) -> Vec3 {
    Vec3::new(
        TILE_SIZE * position.v.x as f32,
        TILE_SIZE * position.v.y as f32,
        z,
    )
}
