use bevy::prelude::*;

/// Component to store information for a diceRoll
#[derive(Component)]
pub struct DiceRoll {
    count: i32,
    sides: i32,
    bonus: i32,
}

/// System to use a diceRoll component to retreive the result of that dice roll
fn Roll(diceRoll: DiceRoll) {
    mut result = diceRoll.bonus;
    for (i: i32 = 0; i < diceRoll.count; i++) {
        result += rand.range(1, diceRoll.sides+1)
    }
    return result;
}
