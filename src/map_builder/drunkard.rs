use bracket_lib::prelude::*;

use crate::map::{map_center, point_within_bounds, TileType, NUM_TILES};
use crate::map_builder::MapArchitect;
use crate::{Map, MapBuilder, RandomNumberGenerator, MAP_HEIGHT, MAP_WIDTH};

const DESIRED_FLOOR: usize = NUM_TILES / 3;
const STAGGER_DISTANCE: usize = 400;

pub struct DrunkardsWalkArchitect {}

impl DrunkardsWalkArchitect {
    fn drunkard(&mut self, start: &Point, rng: &mut RandomNumberGenerator, map: &mut Map) {
        let mut drunkards_position = *start;
        let mut distance_staggered = 0;

        loop {
            let index = map.point2d_to_index(drunkards_position);
            map.tiles[index] = TileType::Floor;

            match rng.range(0, 4) {
                0 => drunkards_position.x -= 1,
                1 => drunkards_position.x += 1,
                2 => drunkards_position.y -= 1,
                _ => drunkards_position.y += 1,
            };

            distance_staggered += 1;

            if !point_within_bounds(drunkards_position) || distance_staggered > STAGGER_DISTANCE {
                break;
            }
        }
    }
}

impl MapArchitect for DrunkardsWalkArchitect {
    fn build(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut builder = MapBuilder::default();
        builder.fill(TileType::Wall);

        self.drunkard(&map_center(), rng, &mut builder.map);

        while builder
            .map
            .tiles
            .iter()
            .filter(|tile| **tile == TileType::Floor)
            .count()
            < DESIRED_FLOOR
        {
            self.drunkard(
                &Point::new(rng.range(0, MAP_WIDTH), rng.range(0, MAP_HEIGHT)),
                rng,
                &mut builder.map,
            );

            let dijkstra_map = DijkstraMap::new(
                MAP_WIDTH,
                MAP_HEIGHT,
                &[builder.map.point2d_to_index(map_center())],
                &builder.map,
                1024.0,
            );

            dijkstra_map
                .map
                .iter()
                .enumerate()
                .filter(|(_, distance)| *distance > &2000.0)
                .for_each(|(index, _)| builder.map.tiles[index] = TileType::Wall);
        }

        builder.wall_map();

        builder.player_start = map_center();
        builder.amulet_position = builder.find_most_distant_point();
        builder.monster_spawns = builder.spawn_monsters(rng);

        builder
    }
}
