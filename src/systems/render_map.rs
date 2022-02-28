use bracket_lib::prelude::*;
use legion::world::SubWorld;
use legion::{component, system, IntoQuery};

use crate::components::{FieldOfView, Player};
use crate::map::{point_to_index, point_within_bounds, TileType};
use crate::{Camera, Map, MAP_LAYER};

#[system]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn render_map(ecs: &SubWorld, #[resource] map: &Map, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(MAP_LAYER);

    let fov = <&FieldOfView>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .next()
        .unwrap();

    for y in camera.viewport.y1..=camera.viewport.y2 {
        for x in camera.viewport.x1..=camera.viewport.x2 {
            let point = Point::new(x, y);
            let offset = Point::new(camera.viewport.x1, camera.viewport.y1);

            if point_within_bounds(point) && fov.visible_tiles.contains(&point) {
                let index = point_to_index(point);
                let glyph = match map.tiles[index] {
                    TileType::Floor => to_cp437('.'),
                    TileType::Wall => to_cp437('#'),
                };

                draw_batch.set(point - offset, ColorPair::new(WHITE, BLACK), glyph);
            }
        }
    }

    draw_batch.submit(0).expect("failed to submit draw batch");
}
