use bracket_lib::prelude::*;
use legion::systems::CommandBuffer;
use legion::world::SubWorld;
use legion::{system, Entity, IntoQuery};

use crate::components::{RandomMovement, WantsToMove};

#[system]
#[read_component(Point)]
#[read_component(RandomMovement)]
pub fn move_randomly(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    <(Entity, &Point, &RandomMovement)>::query()
        .iter_mut(ecs)
        .for_each(|(entity, position, _)| {
            let mut rng = RandomNumberGenerator::new();

            let delta = match rng.range::<i32>(0, 4) {
                0 => Point::new(-1, 0),
                1 => Point::new(1, 0),
                2 => Point::new(0, -1),
                _ => Point::new(0, 1),
            };
            let destination = *position + delta;

            commands.push((
                (),
                WantsToMove {
                    entity: *entity,
                    destination,
                },
            ));
        });
}
