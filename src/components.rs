use bracket_lib::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Enemy;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Player;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct RandomMovement;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}
