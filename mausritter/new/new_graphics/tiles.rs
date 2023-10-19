use bevy::prelude::*;

use crate::board::components::{ Position, Tile };
use super::{ GraphicsAssets, Renderable, TILE_SIZE, TILE_Z };

pub fn spawn_tile_renderer(
    mut commands: Commands,
    query: Query<(Entity, &Position, &Renderable), Added<Tile>>,
    assets: Res<GraphicsAssets>
) {
    for (entity, position, renderable) in query.iter() {
        let sprite_idx = renderable.sprite_idx
        let mut sprite = TextureAtlasSprite::new(sprite_idx);
        sprite.custom_size = renderable.size;
        sprite.color = renderable.color;
        let v = super::get_world_position(&position, TILE_Z);
        commands.entity(entity)
            .insert(
                SpriteSheetBundle {
                    sprite,
                    texture_atlas: assets.sprite_texture.clone(),
                    transform: Transform::from_translation(v),
                    ..Default::default()
                }
            );
    }
}