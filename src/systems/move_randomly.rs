use bracket_lib::prelude::*;
use legion::systems::CommandBuffer;
use legion::world::SubWorld;
use legion::{system, Entity, EntityStore, IntoQuery};

use crate::components::{Health, Player, RandomMovement, WantsToAttack, WantsToMove};

#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Point)]
#[read_component(RandomMovement)]
pub fn move_randomly(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    <(Entity, &Point, &RandomMovement)>::query()
        .iter(ecs)
        .for_each(|(entity, position, _)| {
            let mut rng = RandomNumberGenerator::new();

            let delta = match rng.range::<i32>(0, 4) {
                0 => Point::new(-1, 0),
                1 => Point::new(1, 0),
                2 => Point::new(0, -1),
                _ => Point::new(0, 1),
            };
            let destination = *position + delta;

            let mut has_attacked = false;

            <(Entity, &Point, &Health)>::query()
                .iter(ecs)
                .filter(|(_, target_position, _)| **target_position == destination)
                .for_each(|(victim, _, _)| {
                    if ecs
                        .entry_ref(*victim)
                        .unwrap()
                        .get_component::<Player>()
                        .is_ok()
                    {
                        commands.push((
                            (),
                            WantsToAttack {
                                attacker: *entity,
                                victim: *victim,
                            },
                        ));

                        has_attacked = true;
                    }
                });

            if !has_attacked {
                commands.push((
                    (),
                    WantsToMove {
                        entity: *entity,
                        destination,
                    },
                ));
            }
        });
}
