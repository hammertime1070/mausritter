use bevy::prelude::*;

mod assets;
mod board;
mod camera;
mod globals;
mod graphics;
mod states;
mod vectors;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(
            WindowPlugin {
                primary_window: Some(Window {
                    resolution: (
                        globals::WINDOW_WIDTH,
                        globals::WINDOW_HEIGHT
                    ).into(),
                    ..Default::default()
                }),
                ..Default::default()<
            }
        ).set(
            ImagePlugin::default_nearest()
        )
    )
        .add_resource(Msaa::Off)
        .add_state::<states::MainState>()
        .add_plugin(assets::AssetPlugin)
        .add_plugin(board::BoardPlugin)
        .add_plugin(graphics::GraphicsPlugin)
        .add_systems(Startup, (setup, camera::setup))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(DiceRoll {
        count: 1,
        sides: 6,
        bonus: 0,
    });
    commands.spawn(Strength { score: AbilityScore::new(10) });
    commands.spawn(Dexterity { score: AbilityScore::new(12) });
    commands.spawn(Willpower { score: AbilityScore::new(14) });
}

#[derive(Component)]
struct NewMouse;

fn spawn_mouse(mut commands: Commands) {
    commands.spawn(MouseBundle {..Default::default()}).insert(NewMouse);
}

fn print_new_mice_attributes(
    mut commands: Commands,
    query: Query<(Entity, &HP, &Pips, &Strength, &Dexterity, &Willpower), With<NewMouse>>,
) {
    for (entity, hp, pips, strength, dexterity, willpower) in query.iter() {
        println!("New Mouse Spawned:");
        println!("HP: {:?}", hp.max);
        println!("Pips: {:?}", pips.0);
        println!("Strength: {:?}", strength.score.value);
        println!("Dexterity: {:?}", dexterity.score.value);
        println!("Willpower: {:?}", willpower.score.value);

        // Remove the NewlySpawned marker after processing
        commands.entity(entity).remove::<NewMouse>();
    }
}
