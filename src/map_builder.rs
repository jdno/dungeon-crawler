use std::cmp::{max, min};

use bracket_lib::prelude::*;

use crate::map::{point_to_index, try_point_to_index, TileType};
use crate::{Map, SCREEN_HEIGHT, SCREEN_WIDTH};

const NUM_ROOMS: usize = 20;

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub player_start: Point,
}

impl MapBuilder {
    pub fn new(rng: &mut RandomNumberGenerator) -> Self {
        let mut builder = Self {
            map: Map::new(),
            rooms: Vec::new(),
            player_start: Point::zero(),
        };

        builder.fill(TileType::Wall);
        builder.generate_random_rooms(rng);
        builder.connect_rooms_with_corridors(rng);
        builder.player_start = builder.rooms[0].center();

        builder
    }

    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    fn generate_random_rooms(&mut self, rng: &mut RandomNumberGenerator) {
        while self.rooms.len() < NUM_ROOMS {
            let room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH - 10),
                rng.range(1, SCREEN_HEIGHT - 10),
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
                    if (r.x > 0 && r.x < SCREEN_WIDTH) && (r.y > 0 && r.y < SCREEN_HEIGHT) {
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
