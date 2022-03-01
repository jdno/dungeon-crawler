use crate::map::TileType;
use crate::map_builder::MapArchitect;
use crate::{Map, MapBuilder, Point, RandomNumberGenerator, MAP_HEIGHT, MAP_WIDTH};

pub struct EmptyArchitect {}

impl MapArchitect for EmptyArchitect {
    fn build(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut builder = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            monster_spawns: Vec::new(),
            player_start: Point::zero(),
            amulet_position: Point::zero(),
        };

        builder.fill(TileType::Floor);
        builder.player_start = Point::new(MAP_WIDTH / 2, MAP_HEIGHT / 2);
        builder.amulet_position = builder.find_most_distant_point();

        for _ in 0..50 {
            builder.monster_spawns.push(Point::new(
                rng.range(1, MAP_WIDTH),
                rng.range(1, MAP_HEIGHT),
            ));
        }

        builder
    }
}
