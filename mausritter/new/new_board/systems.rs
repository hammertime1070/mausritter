use bevy::prelude::*;
use std::collections::HashMap;

use crate::vectors::Vector2Int;
use crate::new_graphics::Renderable;

use super::CurrentBoard;
use super::components::{Position, Tile};

pub fn spawn_map(
    mut commands: Commands,
    mut current: ResMut<CurrentBoard>
) {
    current.tiles = HashMap::new();
    for x in 0..8 {
        for y in 0..8 {
            let v = Vector2Int::new(x,y);
            let tile = commands.spawn((
                Position { v },
                Tile,
                Renderable {
                    sprite_idx: 177,
                    size: Some(Vec2::splat(32.))
                    color: Color::Olive,
                    z: 0.0
                }
            ))
            .id();
            current.tiles.insert(v, tile);
        }
    }
}