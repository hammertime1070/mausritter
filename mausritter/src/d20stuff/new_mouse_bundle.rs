use bevy::prelude::*;
use crate::name::*;
use crate::ability_scores::*;
use crate::dice_roll::*;
use crate::ability_scores::*;

#[derive(Bundle)]
pub struct AttributeBundle {
    pub strength: Strength,
    pub dexterity: Dexterity,
    pub willpower: Willpower,
}

#[derive(Bundle)]
pub struct MouseBundle {
    pub attributes: AttributeBundle,
    pub hp: HP,
    pub pips: Pips,
}

impl Default for MouseBundle {
    fn default() -> Self {
        Self {
            hp: HP {max: roll_d6()},
            pips: Pips(roll_d6()),
            attributes: AttributeBundle {
                strength: Strength {score: AbilityScore{value: default_starting_attribute_score()}},
                dexterity: Dexterity {score: AbilityScore{value:default_starting_attribute_score()}},
                willpower: Willpower {score: AbilityScore{value:default_starting_attribute_score()}},
            }
        }
    }
}

pub fn default_starting_attribute_score() ->i32 {
    let dice_roll = DiceRoll {
        count: 1,
        sides: 6,
        bonus: 0,
    };
    let mut rolls = vec![roll(&dice_roll), roll(&dice_roll), roll(&dice_roll)];
    rolls.sort();
    return rolls[1] + rolls[2];
}

pub fn roll_d6() -> i32{
    let dice_roll = DiceRoll {
        count: 1,
        sides: 6,
        bonus: 0,
    };
    return roll(&dice_roll);
}