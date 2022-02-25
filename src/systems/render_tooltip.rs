use bracket_lib::prelude::*;
use legion::world::SubWorld;
use legion::{system, Entity, EntityStore, IntoQuery};

use crate::components::{Health, Name};
use crate::map::NUM_TILES;
use crate::{Camera, HUD_LAYER};

#[system]
#[read_component(Health)]
#[read_component(Name)]
#[read_component(Point)]
pub fn render_tooltip(
    ecs: &SubWorld,
    #[resource] mouse_position: &Point,
    #[resource] camera: &Camera,
) {
    let offset = Point::new(camera.viewport.x1, camera.viewport.y1);
    let map_position = *mouse_position + offset;

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(HUD_LAYER);

    <(Entity, &Point, &Name)>::query()
        .iter(ecs)
        .filter(|(_, position, _)| **position == map_position)
        .for_each(|(entity, _, name)| {
            let screen_position = *mouse_position * 4;
            let display_string =
                if let Ok(health) = ecs.entry_ref(*entity).unwrap().get_component::<Health>() {
                    format!("{} : {} hp", &name.0, health.current)
                } else {
                    name.0.clone()
                };

            draw_batch.print(screen_position, &display_string);
        });

    draw_batch
        .submit(NUM_TILES + 6000)
        .expect("failed to submit draw batch");
}
