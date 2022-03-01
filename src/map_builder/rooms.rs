use crate::map::TileType;
use crate::map_builder::MapArchitect;
use crate::{MapBuilder, RandomNumberGenerator};

pub struct RoomsArchitect {}

impl MapArchitect for RoomsArchitect {
    fn build(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut builder = MapBuilder::default();

        builder.fill(TileType::Wall);
        builder.generate_random_rooms(rng);
        builder.connect_rooms_with_corridors(rng);

        builder.player_start = builder.rooms[0].center();
        builder.amulet_position = builder.find_most_distant_point();

        for room in builder.rooms.iter().skip(1) {
            builder.monster_spawns.push(room.center());
        }

        builder
    }
}
