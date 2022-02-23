use bracket_lib::prelude::*;

use crate::camera::Camera;

pub const MAP_HEIGHT: i32 = 50;
pub const MAP_WIDTH: i32 = 80;
const NUM_TILES: usize = (MAP_WIDTH * MAP_HEIGHT) as usize;

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

    pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        ctx.set_active_console(0);

        for y in camera.viewport.y1..camera.viewport.y2 {
            for x in camera.viewport.x1..camera.viewport.x2 {
                if point_within_bounds(Point::new(x, y)) {
                    let index = coordinate_to_index(x, y);

                    match self.tiles[index] {
                        TileType::Floor => ctx.set(
                            x - camera.viewport.x1,
                            y - camera.viewport.y1,
                            YELLOW,
                            BLACK,
                            to_cp437('.'),
                        ),
                        TileType::Wall => ctx.set(
                            x - camera.viewport.x1,
                            y - camera.viewport.y1,
                            GREEN,
                            BLACK,
                            to_cp437('#'),
                        ),
                    };
                }
            }
        }
    }
}

pub fn coordinate_to_index(x: i32, y: i32) -> usize {
    ((y * MAP_WIDTH) + x) as usize
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
    (point.x >= 0 && point.x < MAP_WIDTH) && (point.y >= 0 && point.y < MAP_HEIGHT)
}
