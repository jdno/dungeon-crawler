use bracket_lib::prelude::*;
use legion::Entity;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Enemy;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Player;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct RandomMovement;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Point,
}
