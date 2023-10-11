use bevy::prelude::*;

mod diceRoll;
use diceRoll::{DiceRoll, dice_roll_system};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(dice_roll_system.system())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn().insert(DiceRoll {
        count: 1,
        sides: 6,
        bonus: 0,
    });
}