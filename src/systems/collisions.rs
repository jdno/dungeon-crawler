use bracket_lib::prelude::*;
use legion::systems::CommandBuffer;
use legion::world::SubWorld;
use legion::{component, system, Entity, IntoQuery};

use crate::components::{Enemy, Player};

#[system]
#[read_component(Enemy)]
#[read_component(Player)]
#[read_component(Point)]
pub fn collisions(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut players = <&Point>::query().filter(component::<Player>());
    let player_position = players
        .iter(ecs)
        .next()
        .expect("failed to get player position");

    let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());
    enemies
        .iter(ecs)
        .filter(|(_, position)| *position == player_position)
        .for_each(|(entity, _)| commands.remove(*entity));
}
