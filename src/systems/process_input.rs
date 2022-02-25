use bracket_lib::prelude::*;
use legion::systems::CommandBuffer;
use legion::world::SubWorld;
use legion::{component, system, Entity, IntoQuery};

use crate::components::{Enemy, Player, WantsToAttack, WantsToMove};
use crate::TurnState;

#[system]
#[read_component(Player)]
#[read_component(Point)]
pub fn process_input(
    ecs: &SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::zero(),
        };

        if delta.x != 0 || delta.y != 0 {
            <(Entity, &Point)>::query()
                .filter(component::<Player>())
                .iter(ecs)
                .for_each(|(player_entity, position)| {
                    let destination = *position + delta;
                    let mut has_hit_something = false;

                    <(Entity, &Point)>::query()
                        .filter(component::<Enemy>())
                        .iter(ecs)
                        .filter(|(_, position)| **position == destination)
                        .for_each(|(enemy_entity, _)| {
                            has_hit_something = true;

                            commands.push((
                                (),
                                WantsToAttack {
                                    attacker: *player_entity,
                                    victim: *enemy_entity,
                                },
                            ));
                        });

                    if !has_hit_something {
                        commands.push((
                            (),
                            WantsToMove {
                                entity: *player_entity,
                                destination,
                            },
                        ));
                    }
                });
        }

        *turn_state = TurnState::PlayerTurn;
    }
}
