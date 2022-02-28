use legion::world::SubWorld;
use legion::{component, system, IntoQuery};

use crate::components::{Health, Player};
use crate::TurnState;

#[system]
#[read_component(Health)]
#[read_component(Player)]
pub fn end_turn(ecs: &SubWorld, #[resource] turn_state: &mut TurnState) {
    let mut player_health = <&Health>::query().filter(component::<Player>());

    let mut next_state = match turn_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        _ => *turn_state,
    };

    player_health.iter(ecs).for_each(|health| {
        if health.current < 1 {
            next_state = TurnState::GameOver;
        }
    });

    *turn_state = next_state;
}
