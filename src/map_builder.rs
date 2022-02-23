use bracket_lib::prelude::*;

use crate::map::{point_to_index, TileType};
use crate::{Map, SCREEN_HEIGHT, SCREEN_WIDTH};

const NUM_ROOMS: usize = 20;

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub player_start: Point,
}

impl MapBuilder {
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
}
