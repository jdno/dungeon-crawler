use legion::systems::CommandBuffer;
use legion::world::SubWorld;
use legion::{system, Entity, EntityStore};

use crate::components::Player;
use crate::components::WantsToMove;
use crate::{Camera, Map};

#[system(for_each)]
#[read_component(Player)]
pub fn move_entity(
    ecs: &SubWorld,
    commands: &mut CommandBuffer,
    entity: &Entity,
    want_move: &WantsToMove,
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
) {
    if map.is_enterable_tile(want_move.destination) {
        commands.add_component(want_move.entity, want_move.destination);

        if ecs
            .entry_ref(want_move.entity)
            .unwrap()
            .get_component::<Player>()
            .is_ok()
        {
            camera.on_player_move(want_move.destination);
        }
    }
    commands.remove(*entity);
}
