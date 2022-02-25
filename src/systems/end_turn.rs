use legion::system;

use crate::TurnState;

#[system]
pub fn end_turn(#[resource] turn_state: &mut TurnState) {
    let next_state = match turn_state {
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        TurnState::AwaitingInput => return,
    };

    *turn_state = next_state;
}
