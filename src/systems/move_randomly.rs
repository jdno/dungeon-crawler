use bracket_lib::prelude::*;
use legion::world::SubWorld;
use legion::{system, IntoQuery};

use crate::components::RandomMovement;
use crate::Map;

#[system]
#[read_component(RandomMovement)]
#[write_component(Point)]
pub fn move_randomly(ecs: &mut SubWorld, #[resource] map: &Map) {
    <(&mut Point, &RandomMovement)>::query()
        .iter_mut(ecs)
        .for_each(|(position, _)| {
            let mut rng = RandomNumberGenerator::new();

            let delta = match rng.range(0, 4) {
                0 => Point::new(-1, 0),
                1 => Point::new(1, 0),
                2 => Point::new(0, -1),
                _ => Point::new(0, 1),
            };
            let destination = *position + delta;

            if map.is_enterable_tile(destination) {
                *position = destination;
            }
        });
}
