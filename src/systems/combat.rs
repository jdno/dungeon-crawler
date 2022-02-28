use legion::systems::CommandBuffer;
use legion::world::SubWorld;
use legion::{system, Entity, EntityStore, IntoQuery};

use crate::components::{Health, Player, WantsToAttack};

#[system]
#[read_component(Player)]
#[read_component(WantsToAttack)]
#[write_component(Health)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let victims: Vec<(Entity, Entity)> = <(Entity, &WantsToAttack)>::query()
        .iter(ecs)
        .map(|(message, attack)| (*message, attack.victim))
        .collect();

    victims.iter().for_each(|(message, victim)| {
        let is_player = ecs
            .entry_ref(*victim)
            .unwrap()
            .get_component::<Player>()
            .is_ok();

        if let Ok(mut health) = ecs
            .entry_mut(*victim)
            .unwrap()
            .get_component_mut::<Health>()
        {
            health.current -= 1;

            if health.current < 1 && !is_player {
                commands.remove(*victim);
            }
        }

        commands.remove(*message);
    })
}
