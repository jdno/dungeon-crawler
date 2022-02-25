#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum TurnState {
    AwaitingInput,
    MonsterTurn,
    PlayerTurn,
}
