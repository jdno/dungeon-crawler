use bracket_lib::prelude::*;
use legion::world::SubWorld;
use legion::{component, system, IntoQuery};

use crate::components::{AmuletOfYala, Health, Player};
use crate::TurnState;

#[system]
#[read_component(AmuletOfYala)]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Point)]
pub fn end_turn(ecs: &SubWorld, #[resource] turn_state: &mut TurnState) {
    let mut player = <(&Health, &Point)>::query().filter(component::<Player>());
    let mut amulet = <&Point>::query().filter(component::<AmuletOfYala>());

    let mut next_state = match turn_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        _ => *turn_state,
    };

    let amulet_position = amulet.iter(ecs).next().unwrap();

    player.iter(ecs).for_each(|(health, position)| {
        if health.current < 1 {
            next_state = TurnState::Defeat;
        }
        if position == amulet_position {
            next_state = TurnState::Victory;
        }
    });

    *turn_state = next_state;
}
