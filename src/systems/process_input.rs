use bracket_lib::prelude::*;
use legion::systems::CommandBuffer;
use legion::world::SubWorld;
use legion::{component, system, Entity, EntityStore, IntoQuery};

use crate::components::{Enemy, Health, Player, WantsToAttack, WantsToMove};
use crate::TurnState;

#[system]
#[read_component(Enemy)]
#[read_component(Player)]
#[read_component(Point)]
#[write_component(Health)]
pub fn process_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    let mut players = <(Entity, &Point)>::query().filter(component::<Player>());
    let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());

    if let Some(key) = key {
        let mut did_something = false;

        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::zero(),
        };

        let (player_entity, destination) = players
            .iter(ecs)
            .find_map(|(entity, pos)| Some((*entity, *pos + delta)))
            .unwrap();

        if delta.x != 0 || delta.y != 0 {
            let mut has_hit_something = false;

            enemies
                .iter(ecs)
                .filter(|(_, position)| **position == destination)
                .for_each(|(enemy_entity, _)| {
                    has_hit_something = true;
                    did_something = true;

                    commands.push((
                        (),
                        WantsToAttack {
                            attacker: player_entity,
                            victim: *enemy_entity,
                        },
                    ));
                });

            if !has_hit_something {
                did_something = true;

                commands.push((
                    (),
                    WantsToMove {
                        entity: player_entity,
                        destination,
                    },
                ));
            }
        }

        if !did_something {
            if let Ok(mut health) = ecs
                .entry_mut(player_entity)
                .unwrap()
                .get_component_mut::<Health>()
            {
                health.current = i32::min(health.max, health.current + 1);
            }
        }

        *turn_state = TurnState::PlayerTurn;
    }
}
