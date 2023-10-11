use bevy::prelude::*;

#[derive(Component)]
pub struct Name(String);

#[derive(Component)]
pub struct HP {
    max: u32,
    current: i32,
}

#[derive(Component)]
pub struct Pips(i32);