use bevy::prelude::*;

mod dice_roll;
mod name;
use name::*;
use dice_roll::{DiceRoll, dice_roll_system};
mod ability_scores;
use ability_scores::{AbilityScore, Strength, Dexterity, Willpower, strength_system, dexterity_system, willpower_system};
mod new_mouse_bundle;
use new_mouse_bundle::{ MouseBundle, AttributeBundle };
mod inventory_grid;
use inventory_grid:: {spawn_layout};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, spawn_layout))
        // .add_systems(Update, (spawn_mouse, print_new_mice_attributes))
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