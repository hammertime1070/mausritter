use bevy::prelude::*;

// This may not work but this is ideally how I would like to display renderable shit
// Components
#[derive(Component)]
pub struct Renderable {
    spritesheet: TextureAtlas,
    sprite_idx: i32,
}

// Systems
fn render_renderable(
    mut commands: Commands,
    query: Query<(Entity, &Position), With<Renderable>>,
    assets: Res<GraphicsAssets>,
) {
    for (entity, position) in query.iter() {
        let mut sprite = Renderable.spritesheet(sprite_idx);
        sprite.custom_size = Some(Vec2::splat(TILE_SIZE));
        sprite.color = Color::WHITE;
        let v = super::get_world_position(&position, PIECE_Z);
        commands.entity(entity).insert(SpriteSheetBundle {
            sprite,
            texture_atlas: assets.sprite_texture.clone(),
            transform: Transform::from_translation(v),
            ..Default::default()
        });
    }
}
