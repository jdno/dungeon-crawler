use bracket_lib::prelude::*;

use crate::map::{coordinate_to_index, TileType};
use crate::map_builder::MapArchitect;
use crate::{Map, MapBuilder, MAP_HEIGHT, MAP_WIDTH};

pub struct CellularAutomataArchitect {}

impl CellularAutomataArchitect {
    fn generate_random_noise(&self, rng: &mut RandomNumberGenerator, map: &mut Map) {
        map.tiles.iter_mut().for_each(|tile| {
            let roll = rng.range(0, 100);

            if roll > 55 {
                *tile = TileType::Floor
            } else {
                *tile = TileType::Wall;
            }
        });
    }

    fn iteration(&self, map: &mut Map) {
        let mut tiles = map.tiles.clone();

        for y in 1..MAP_HEIGHT - 1 {
            for x in 1..MAP_WIDTH - 1 {
                let neighbors = self.count_neighbors(x, y, map);
                let index = coordinate_to_index(x, y);

                if neighbors > 4 || neighbors == 0 {
                    tiles[index] = TileType::Wall;
                } else {
                    tiles[index] = TileType::Floor;
                }
            }
        }

        map.tiles = tiles;
    }

    fn count_neighbors(&self, x: i32, y: i32, map: &Map) -> usize {
        let mut neighbors = 0;

        for iy in -1..=1 {
            for ix in -1..=1 {
                if !(ix == 0 && iy == 0)
                    && map.tiles[coordinate_to_index(x + ix, y + iy)] == TileType::Wall
                {
                    neighbors += 1;
                }
            }
        }

        neighbors
    }

    fn find_start(&self, map: &Map) -> Point {
        let center = Point::new(MAP_WIDTH / 2, MAP_HEIGHT / 2);

        let closest_point = map
            .tiles
            .iter()
            .enumerate()
            .filter(|(_, tile)| **tile == TileType::Floor)
            .map(|(index, _)| {
                (
                    index,
                    DistanceAlg::Pythagoras.distance2d(center, map.index_to_point2d(index)),
                )
            })
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(index, _)| index)
            .unwrap();

        map.index_to_point2d(closest_point)
    }
}

impl MapArchitect for CellularAutomataArchitect {
    fn build(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut builder = MapBuilder::default();

        self.generate_random_noise(rng, &mut builder.map);

        for _ in 0..10 {
            self.iteration(&mut builder.map);
        }

        builder.player_start = self.find_start(&builder.map);
        builder.amulet_position = builder.find_most_distant_point();
        builder.monster_spawns = builder.spawn_monsters(rng);

        builder
    }
}
