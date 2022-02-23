use bracket_lib::prelude::*;

use crate::{DISPLAY_HEIGHT, DISPLAY_WIDTH};

pub struct Camera {
    viewport: Rect,
}

impl Camera {
    pub fn new(player_position: Point) -> Self {
        Self {
            viewport: viewport_from_point(player_position),
        }
    }

    pub fn on_player_move(&mut self, player_position: Point) {
        self.viewport = viewport_from_point(player_position);
    }
}

fn viewport_from_point(point: Point) -> Rect {
    Rect::with_size(
        point.x - DISPLAY_WIDTH / 2,
        point.y - DISPLAY_HEIGHT / 2,
        DISPLAY_WIDTH,
        DISPLAY_HEIGHT,
    )
}
