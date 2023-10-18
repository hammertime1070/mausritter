use bevy::prelude::*;

use crate::board::components::Position;
use crate::states::MainState;
use crate::vectors::Vector2Int;
use crate::new_graphics::Renderable;

pub mod components;

pub struct PiecesPlugin;

impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MainState::Game), spawn_npcs);
    }
}

pub fn spawn_npcs(mut commands: Commands) {
    commands.spawn((
        components::Actor::default(),
        components::Piece {
            kind: "NPC".to_string(),
        },
        Position {
            v: Vector2Int::new(3, 5),
        },
        components::Walk,
        renderable {
            sprite_idx: 63,
            size: Some(Vec2::splat(32.)),
            color:Color::White,
            z: 10.,
        }
    ));
    commands.spawn((
        components::Actor::default(),
        components::Piece {
            kind: "NPC".to_string(),
        },
        Position {
            v: Vector2Int::new(5, 5),
        },
        components::Walk,
        renderable {
            sprite_idx: 63,
            size: Some(Vec2::splat(TILE_SIZE)),
            color:Color::White,
            z: 10.,
        }
    ));
}
