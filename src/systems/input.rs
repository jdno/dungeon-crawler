use bracket_lib::prelude::*;
use legion::world::SubWorld;
use legion::{component, system, IntoQuery};

use crate::components::Player;
use crate::{Camera, Map};

#[system]
#[read_component(Player)]
#[write_component(Point)]
pub fn input(
    ecs: &mut SubWorld,
    #[resource] map: &Map,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] camera: &mut Camera,
) {
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::zero(),
        };

        if delta.x != 0 || delta.y != 0 {
            let mut players = <&mut Point>::query().filter(component::<Player>());

            players.iter_mut(ecs).for_each(|position| {
                let destination = *position + delta;

                if map.is_enterable_tile(destination) {
                    *position = destination;
                    camera.on_player_move(destination);
                }
            })
        }
    }
}
