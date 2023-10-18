use bevy::prelude::*;

#[derive(Component)]
pub struct Name(pub String);

#[derive(Component)]
pub struct HP {
    pub max: i32,
}

#[derive(Component)]
pub struct Pips(pub i32);