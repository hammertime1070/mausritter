use bevy::prelude::*;

#[derive(Bundle)]
struct AttributeBundle {
    strength: Strength { score: AbilityScore::new()},
    dexterity: Dexterity { score: AbilityScore::new()},
    willpower: Willpower { score: AbilityScore::new()},
}

#[derive(Bundle)]
struct MouseBundle {
    name: Name,
    attributes: AttributeBundle,
    hp: HP,
    pips: Pips,
}

impl Default for MouseBundle {
    fn default() -> Self {
        name: Name("Tom"),
        hp: default_starting_hp(),
        pips: default_starting_pips(),
        attributes: AttributeBundle {
            strength: default_starting_attribute_score(),
            dexterity: default_starting_attribute_score(),
            willpower: default_starting_attribute_score(),
        }
    }
}

pub fn default_starting_attribute_score() {
    let dice_roll = DiceRoll {
        count: 1,
        sides: 6,
        bonus: 0,
    }
    let mut rolls = vec![roll(dice_roll), roll(dice_roll), roll(dice_roll)];
    rolls.sort();
    return rolls[1] + rolls[2]
}

pub fn default_starting_hp() {
    pass
}

pub fn default_starting_pips() {
    pass
}