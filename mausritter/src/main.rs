use bevy::prelude::*;

mod dice_roll;
use dice_roll::{DiceRoll, dice_roll_system};
mod ability_scores;
use ability_scores::{AbilityScore, Strength, Dexterity, Willpower, strength_system, dexterity_system, willpower_system};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, dice_roll_system)
        .add_systems(Update, strength_system)
        .add_systems(Update, dexterity_system)
        .add_systems(Update, willpower_system)
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