use bevy::prelude::*;

use crate::board::components::Position;
use crate::pieces::components::{Actor, Piece};
use crate::states::MainState;
use crate::vectors::Vector2Int;
use crate::new_graphics::Renderable;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MainState::Game), spawn_player);
    }
}

#[derive(Component)]
pub struct Player;

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Actor::default(),
        Player,
        Piece {
            kind: "Player".to_string(),
        },
        Position {
            v: Vector2Int::new(0, 0),
        },
        Renderable {
            sprite_idx: 1,
            size: Some(Vec2::splat(32.)),
            color:Color::White;
            z: 10.,
        }
    ));
}
