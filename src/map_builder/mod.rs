use std::cmp::{max, min};

use bracket_lib::prelude::*;

use crate::map::{point_to_index, try_point_to_index, Map, TileType, MAP_HEIGHT, MAP_WIDTH};

pub use self::empty::*;
pub use self::rooms::*;

mod empty;
mod rooms;

const NUM_ROOMS: usize = 20;

pub trait MapArchitect {
    fn build(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder;
}

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub monster_spawns: Vec<Point>,
    pub player_start: Point,
    pub amulet_position: Point,
}

impl MapBuilder {
    pub fn new(rng: &mut RandomNumberGenerator) -> Self {
        let mut architect = RoomsArchitect {};
        architect.build(rng)
    }

    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    fn find_most_distant_point(&self) -> Point {
        let dijkstra_map = DijkstraMap::new(
            MAP_WIDTH,
            MAP_HEIGHT,
            &[point_to_index(self.player_start)],
            &self.map,
            1024.0,
        );

        const UNREACHABLE: &f32 = &f32::MAX;
        self.map.index_to_point2d(
            dijkstra_map
                .map
                .iter()
                .enumerate()
                .filter(|(_, distance)| *distance < UNREACHABLE)
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                .unwrap()
                .0,
        )
    }

    fn generate_random_rooms(&mut self, rng: &mut RandomNumberGenerator) {
        while self.rooms.len() < NUM_ROOMS {
            let room = Rect::with_size(
                rng.range(1, MAP_WIDTH - 10),
                rng.range(1, MAP_HEIGHT - 10),
                rng.range(2, 10),
                rng.range(2, 10),
            );

            let mut overlap = false;

            for r in self.rooms.iter() {
                if r.intersect(&room) {
                    overlap = true;
                    break;
                }
            }

            if !overlap {
                room.for_each(|r| {
                    if (r.x > 0 && r.x < MAP_WIDTH) && (r.y > 0 && r.y < MAP_HEIGHT) {
                        let index = point_to_index(r);
                        self.map.tiles[index] = TileType::Floor;
                    }
                });

                self.rooms.push(room);
            }
        }
    }

    fn connect_rooms_with_corridors(&mut self, rng: &mut RandomNumberGenerator) {
        let mut rooms = self.rooms.clone();

        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

        for (i, room) in rooms.iter().enumerate().skip(1) {
            let previous = rooms[i - 1].center();
            let next = room.center();

            if rng.range(0, 2) == 1 {
                self.build_horizontal_tunnel(previous.x, next.x, previous.y);
                self.build_vertical_tunnel(next.x, previous.y, next.y);
            } else {
                self.build_vertical_tunnel(previous.x, previous.y, next.y);
                self.build_horizontal_tunnel(previous.x, next.x, next.y);
            }
        }
    }

    fn build_horizontal_tunnel(&mut self, start: i32, end: i32, y: i32) {
        for x in min(start, end)..=max(start, end) {
            if let Some(index) = try_point_to_index(Point::new(x, y)) {
                self.map.tiles[index as usize] = TileType::Floor;
            }
        }
    }

    fn build_vertical_tunnel(&mut self, x: i32, start: i32, end: i32) {
        for y in min(start, end)..=max(start, end) {
            if let Some(index) = try_point_to_index(Point::new(x, y)) {
                self.map.tiles[index as usize] = TileType::Floor;
            }
        }
    }
}
