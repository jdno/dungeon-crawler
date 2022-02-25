use bracket_lib::prelude::*;
use legion::systems::CommandBuffer;
use legion::world::SubWorld;
use legion::{component, system, Entity, IntoQuery};

use crate::components::{Player, WantsToMove};
use crate::TurnState;

#[system]
#[read_component(Player)]
#[read_component(Point)]
pub fn process_input(
    ecs: &mut SubWorld,
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

        <(Entity, &Point)>::query()
            .filter(component::<Player>())
            .iter_mut(ecs)
            .for_each(|(entity, position)| {
                let destination = *position + delta;

                commands.push((
                    (),
                    WantsToMove {
                        entity: *entity,
                        destination,
                    },
                ));
            });

        *turn_state = TurnState::PlayerTurn;
    }
}
