use legion::systems::CommandBuffer;
use legion::world::SubWorld;
use legion::{system, Entity, EntityStore};

use crate::components::WantsToMove;
use crate::components::{FieldOfView, Player};
use crate::map::point_to_index;
use crate::{Camera, Map};

#[system(for_each)]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn move_entity(
    ecs: &SubWorld,
    commands: &mut CommandBuffer,
    entity: &Entity,
    want_move: &WantsToMove,
    #[resource] map: &mut Map,
    #[resource] camera: &mut Camera,
) {
    if map.is_enterable_tile(want_move.destination) {
        commands.add_component(want_move.entity, want_move.destination);

        if let Ok(entry) = ecs.entry_ref(want_move.entity) {
            if let Ok(fov) = entry.get_component::<FieldOfView>() {
                commands.add_component(want_move.entity, fov.clone_dirty());

                if entry.get_component::<Player>().is_ok() {
                    camera.on_player_move(want_move.destination);
                    fov.visible_tiles.iter().for_each(|position| {
                        map.revealed_tiles[point_to_index(*position)] = true;
                    })
                }
            }
        }
    }

    commands.remove(*entity);
}
