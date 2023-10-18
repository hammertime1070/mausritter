use bevy::prelude::*;

mod actions;
mod assets;
mod board;
mod camera;
mod globals;
mod graphics;
mod input;
mod manager;
mod pieces;
mod player;
mod states;
mod vectors;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<states::MainState>()
        .add_state::<states::GameState>()
        .add_plugins(actions::ActionsPlugin)
        .add_plugins(assets::AssetPlugin)
        .add_plugins(board::BoardPlugin)
        .add_plugins(graphics::GraphicsPlugin)
        .add_plugins(input::InputPlugin)
        .add_plugins(manager::ManagerPlugin)
        .add_plugins(pieces::PiecesPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_systems(Startup, camera::setup)
        .add_systems(Update, log_game_state)
        .run();
}

fn log_game_state(state: Res<State<states::GameState>>) {
    info!("Current GameState: {:?}", state);
}

// fn setup(mut commands: Commands) {
//     commands.spawn(DiceRoll {
//         count: 1,
//         sides: 6,
//         bonus: 0,
//     });
//     commands.spawn(Strength { score: AbilityScore::new(10) });
//     commands.spawn(Dexterity { score: AbilityScore::new(12) });
//     commands.spawn(Willpower { score: AbilityScore::new(14) });
// }

// #[derive(Component)]
// struct NewMouse;

// fn spawn_mouse(mut commands: Commands) {
//     commands.spawn(MouseBundle {..Default::default()}).insert(NewMouse);
// }

// fn print_new_mice_attributes(
//     mut commands: Commands,
//     query: Query<(Entity, &HP, &Pips, &Strength, &Dexterity, &Willpower), With<NewMouse>>,
// ) {
//     for (entity, hp, pips, strength, dexterity, willpower) in query.iter() {
//         println!("New Mouse Spawned:");
//         println!("HP: {:?}", hp.max);
//         println!("Pips: {:?}", pips.0);
//         println!("Strength: {:?}", strength.score.value);
//         println!("Dexterity: {:?}", dexterity.score.value);
//         println!("Willpower: {:?}", willpower.score.value);

//         // Remove the NewlySpawned marker after processing
//         commands.entity(entity).remove::<NewMouse>();
//     }
// }
