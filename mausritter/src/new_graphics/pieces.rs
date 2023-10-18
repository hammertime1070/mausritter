use bevy::prelude::*;

use super::{GraphicsAssets, Renderable, PIECE_SPEED, PIECE_Z, POSITION_TOLERANCE, TILE_SIZE};
use crate::board::components::Position;
use crate::pieces::components::Piece;



pub fn update_piece_position(
    mut query: Query<(&Position, &mut Transform), With<Piece>>,
    time: Res<Time>,
    mut ev_wait: EventWriter<super::GraphicsWaitEvent>,
) {
    let mut animating = false;
    for (position, mut transform) in query.iter_mut() {
        let target = super::get_world_position(&position, PIECE_Z);
        let d = (target - transform.translation).length();
        if d > POSITION_TOLERANCE {
            transform.translation = transform
                .translation
                .lerp(target, PIECE_SPEED * time.delta_seconds());
            animating = true;
        } else {
            transform.translation = target;
        }
    }
    if animating {
        ev_wait.send(super::GraphicsWaitEvent);
    }
}

pub fn spawn_piece_renderer(
    mut commands: Commands,
    query: Query<(Entity, &Position, &Piece, &Renderable), Added<Piece>>,
    assets: Res<GraphicsAssets>,
) {
    for (entity, position, piece, renderable) in query.iter() {
        let sprite_idx = renderable.sprite_idx
        let mut sprite = TextureAtlasSprite::new(sprite_idx);
        sprite.custom_size = renderable.size;
        sprite.color = renderable.color;
        let v = super::get_world_position(&position, renderable.z);
        commands.entity(entity).insert(SpriteSheetBundle {
            sprite,
            texture_atlas: assets.sprite_texture.clone(),
            transform: Transform::from_translation(v),
            ..Default::default()
        });
    }
}
