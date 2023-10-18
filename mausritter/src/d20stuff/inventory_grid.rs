use bevy::prelude::*;

pub fn spawn_layout(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    // Main content grid
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(0.0),
                left: Val::Percent(15.0), // Centering it (100 - 70) / 2
                width: Val::Percent(70.0),
                height: Val::Px(224.0),
                display: Display::Grid,
                grid_template_columns: RepeatedGridTrack::flex(5, 1.0),
                grid_template_rows: RepeatedGridTrack::flex(2, 1.0),
                row_gap: Val::Px(12.0),
                column_gap: Val::Px(12.0),
                ..Default::default()
            },
            background_color: BackgroundColor(Color::DARK_GRAY),
            ..Default::default()
        })
        .with_children(|builder| {
            item_rect(builder, Color::ORANGE);
            item_rect(builder, Color::BISQUE);
            item_rect(builder, Color::BLUE);
            item_rect(builder, Color::CRIMSON);
            item_rect(builder, Color::CYAN);

            item_rect(builder, Color::ORANGE_RED);
            item_rect(builder, Color::DARK_GREEN);
            item_rect(builder, Color::FUCHSIA);
            item_rect(builder, Color::TEAL);
            item_rect(builder, Color::ALICE_BLUE);
        });
}

fn item_rect(builder: &mut ChildBuilder, color: Color) {
    builder
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                padding: UiRect::all(Val::Px(3.0)),
                ..Default::default()
            },
            background_color: BackgroundColor(Color::BLACK),
            ..Default::default()
        })
        .with_children(|builder| {
            builder.spawn(NodeBundle {
                background_color: BackgroundColor(color),
                ..Default::default()
            });
        });
}