use bracket_lib::prelude::*;
use specs::{Component, DenseVecStorage};

#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Renderable {
    pub glyph: FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}

#[derive(Component, Debug)]
pub struct Player {}

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}
