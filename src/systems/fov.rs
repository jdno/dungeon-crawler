use bracket_lib::prelude::*;
use legion::world::SubWorld;
use legion::{system, IntoQuery};

use crate::components::FieldOfView;
use crate::Map;

#[system]
#[read_component(Point)]
#[write_component(FieldOfView)]
pub fn field_of_view(ecs: &mut SubWorld, #[resource] map: &Map) {
    <(&Point, &mut FieldOfView)>::query()
        .iter_mut(ecs)
        .filter(|(_, fov)| fov.is_dirty)
        .for_each(|(position, mut fov)| {
            fov.visible_tiles = field_of_view_set(*position, fov.radius, map);
            fov.is_dirty = false;
        });
}
