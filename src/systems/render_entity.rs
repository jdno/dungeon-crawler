use bracket_lib::prelude::*;
use legion::world::SubWorld;
use legion::{component, system, IntoQuery};

use crate::components::{FieldOfView, Player, Render};
use crate::map::NUM_TILES;
use crate::{Camera, ENTITY_LAYER};

#[system]
#[read_component(FieldOfView)]
#[read_component(Player)]
#[read_component(Point)]
#[read_component(Render)]
pub fn render_entity(ecs: &SubWorld, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(ENTITY_LAYER);

    let fov = <&FieldOfView>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .next()
        .unwrap();

    let offset = Point::new(camera.viewport.x1, camera.viewport.y1);

    <(&Point, &Render)>::query()
        .iter(ecs)
        .filter(|(position, _)| fov.visible_tiles.contains(position))
        .for_each(|(position, render)| {
            draw_batch.set(*position - offset, render.color, render.glyph);
        });

    draw_batch
        .submit(NUM_TILES + 1000)
        .expect("failed to submit draw batch");
}
