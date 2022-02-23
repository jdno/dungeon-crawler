use bracket_lib::prelude::*;

use crate::camera::Camera;
use crate::map::Map;

pub struct Player {
    pub position: Point,
}

impl Player {
    pub fn new(position: Point) -> Self {
        Self { position }
    }

    pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        ctx.set_active_console(1);

        ctx.set(
            self.position.x - camera.viewport.x1,
            self.position.y - camera.viewport.y1,
            WHITE,
            BLACK,
            to_cp437('@'),
        );
    }

    pub fn update(&mut self, ctx: &mut BTerm, map: &Map, camera: &mut Camera) {
        if let Some(key) = ctx.key {
            let delta = match key {
                VirtualKeyCode::Left => Point::new(-1, 0),
                VirtualKeyCode::Right => Point::new(1, 0),
                VirtualKeyCode::Up => Point::new(0, -1),
                VirtualKeyCode::Down => Point::new(0, 1),
                _ => Point::zero(),
            };

            let next_position = self.position + delta;

            if map.is_enterable_tile(next_position) {
                self.position = next_position;
                camera.on_player_move(next_position);
            }
        }
    }
}
