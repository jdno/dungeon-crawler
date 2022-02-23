use bracket_lib::prelude::*;

use crate::map::TileType;
use crate::Map;

const NUM_ROOMS: usize = 20;

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub player_start: Point,
}
