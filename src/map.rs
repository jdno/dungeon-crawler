use bracket_lib::prelude::*;

pub const MAP_HEIGHT: i32 = 50;
pub const MAP_WIDTH: i32 = 80;
pub const NUM_TILES: usize = (MAP_WIDTH * MAP_HEIGHT) as usize;

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

    pub fn is_valid_step(&self, position: Point, delta: Point) -> Option<usize> {
        let destination = position + delta;

        if point_within_bounds(destination) && self.is_enterable_tile(destination) {
            let index = point_to_index(destination);
            Some(index)
        } else {
            None
        }
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(MAP_WIDTH, MAP_HEIGHT)
    }

    fn in_bounds(&self, point: Point) -> bool {
        point_within_bounds(point)
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, index: usize) -> bool {
        self.tiles[index] != TileType::Floor
    }

    fn get_available_exits(&self, index: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let position = self.index_to_point2d(index);

        if let Some(index) = self.is_valid_step(position, Point::new(-1, 0)) {
            exits.push((index, 1.0));
        }

        if let Some(index) = self.is_valid_step(position, Point::new(1, 0)) {
            exits.push((index, 1.0));
        }

        if let Some(index) = self.is_valid_step(position, Point::new(0, -1)) {
            exits.push((index, 1.0));
        }

        if let Some(index) = self.is_valid_step(position, Point::new(0, 1)) {
            exits.push((index, 1.0));
        }

        exits
    }

    fn get_pathing_distance(&self, start: usize, destination: usize) -> f32 {
        DistanceAlg::Pythagoras.distance2d(
            self.index_to_point2d(start),
            self.index_to_point2d(destination),
        )
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
