use bevy::prelude::*;

pub struct AbilityScore {
    value: i32,
}

impl AbilityScore {
    pub fn new(value: i32) -> Self {
        AbilityScore { value }
    }

    pub fn modifier(&self) -> i32 {
        self.value / 2 - 5
    }
}

pub enum Attribute {
    Strength,
    Dexterity,
    Willpower,
}

#[derive(Component)]
pub struct Strength {
    pub score: AbilityScore,
}
#[derive(Component)]
pub struct Dexterity {
    pub score: AbilityScore,
}
#[derive(Component)]
pub struct Willpower {
    pub score: AbilityScore,
}

pub fn strength_system(query: Query<&Strength>) {
    for strength in query.iter() {
        println!("Strength: {}, Modifier {}", strength.score.value, strength.score.modifier())
    }
}
pub fn dexterity_system(query: Query<&Dexterity>) {
    for dexterity in query.iter() {
        println!("Dexterity: {}, Modifier {}", dexterity.score.value, dexterity.score.modifier())
    }
}
pub fn willpower_system(query: Query<&Willpower>) {
    for willpower in query.iter() {
        println!("Willpower: {}, Modifier {}", willpower.score.value, willpower.score.modifier())
    }
}