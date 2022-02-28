use bracket_lib::prelude::*;
use legion::systems::CommandBuffer;
use legion::world::SubWorld;
use legion::{system, Entity, IntoQuery};

use crate::components::{ChasesPlayer, FieldOfView, Health, Player, WantsToAttack, WantsToMove};
use crate::map::point_to_index;
use crate::{Map, MAP_HEIGHT, MAP_WIDTH};

#[system]
#[read_component(ChasesPlayer)]
#[read_component(FieldOfView)]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Point)]
pub fn chase_player(ecs: &SubWorld, commands: &mut CommandBuffer, #[resource] map: &Map) {
    let mut chasing_monsters = <(Entity, &Point, &ChasesPlayer, &FieldOfView)>::query();
    let mut player = <(Entity, &Point, &Player)>::query();

    let (player_entity, player_position, _) = player.iter(ecs).next().unwrap();
    let player_index = point_to_index(*player_position);

    let search_targets = vec![player_index];
    let dijkstra_map = DijkstraMap::new(MAP_WIDTH, MAP_HEIGHT, &search_targets, map, 1024.0);

    chasing_monsters
        .iter(ecs)
        .for_each(|(entity, position, _, fov)| {
            if !fov.visible_tiles.contains(player_position) {
                return;
            }

            let index = point_to_index(*position);

            if let Some(destination) = DijkstraMap::find_lowest_exit(&dijkstra_map, index, map) {
                let distance = DistanceAlg::Pythagoras.distance2d(*position, *player_position);

                if distance > 1.2 {
                    commands.push((
                        (),
                        WantsToMove {
                            entity: *entity,
                            destination: map.index_to_point2d(destination),
                        },
                    ));
                } else {
                    commands.push((
                        (),
                        WantsToAttack {
                            attacker: *entity,
                            victim: *player_entity,
                        },
                    ));
                };
            }
        });
}
