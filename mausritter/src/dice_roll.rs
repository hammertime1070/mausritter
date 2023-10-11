use bevy::prelude::*;
use rand::Rng;

/// Component to store information for a diceRoll
#[derive(Component)]
pub struct DiceRoll {
    pub count: u32,
    pub sides: u32,
    pub bonus: i32,
}

/// System to use a diceRoll component to retreive the result of that dice roll
pub fn dice_roll_system(query: Query<&DiceRoll>) {
    let mut rng = rand::thread_rng();

    for dice_roll in query.iter() {
        let mut result = dice_roll.bonus;

        for _ in 0..dice_roll.count {
            result += rng.gen_range(1..=dice_roll.sides as i32);
        }
        println!("Rolled dice ({}d{} + {}): Result = {}", dice_roll.count, dice_roll.sides, dice_roll.bonus, result);
    }
}
