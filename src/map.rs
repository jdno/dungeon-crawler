use bracket_lib::prelude::*;

use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum TileType {
    Floor,
    Wall,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    pub fn is_enterable_tile(&self, point: Point) -> bool {
        point_within_bounds(point) && self.tiles[point_to_index(point)] == TileType::Floor
    }

    pub fn render(&self, ctx: &mut BTerm) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let index = coordinate_to_index(x, y);

                match self.tiles[index] {
                    TileType::Floor => ctx.set(x, y, YELLOW, BLACK, to_cp437('.')),
                    TileType::Wall => ctx.set(x, y, GREEN, BLACK, to_cp437('#')),
                };
            }
        }
    }
}

pub fn coordinate_to_index(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

pub fn point_to_index(point: Point) -> usize {
    coordinate_to_index(point.x, point.y)
}

pub fn try_point_to_index(point: Point) -> Option<usize> {
    if point_within_bounds(point) {
        Some(point_to_index(point))
    } else {
        None
    }
}

pub fn point_within_bounds(point: Point) -> bool {
    (point.x >= 0 && point.x < SCREEN_WIDTH) && (point.y >= 0 && point.y < SCREEN_HEIGHT)
}
