use crate::prelude::*;

#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Renderable {
    pub glyph: FontCharType,
    pub color: ColorPair,
}

#[derive(Component, Debug)]
pub struct Player {}

#[derive(Component)]
pub struct LeftMover {}