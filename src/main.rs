use bracket_lib::prelude::*;

use crate::camera::Camera;
use crate::map::{Map, MAP_HEIGHT, MAP_WIDTH};
use crate::map_builder::MapBuilder;
use crate::player::Player;

mod camera;
mod map;
mod map_builder;
mod player;

const DISPLAY_HEIGHT: i32 = MAP_HEIGHT / 2;
const DISPLAY_WIDTH: i32 = MAP_WIDTH / 2;

struct State {
    camera: Camera,
    map: Map,
    player: Player,
}

impl State {
    fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);

        Self {
            camera: Camera::new(map_builder.player_start),
            map: map_builder.map,
            player: Player::new(map_builder.player_start),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        self.player.update(ctx, &self.map);

        self.map.render(ctx);
        self.player.render(ctx);
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .build()?;

    main_loop(context, State::new())
}
