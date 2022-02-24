use bracket_lib::prelude::*;
use legion::{Resources, Schedule, World};

use crate::camera::Camera;
use crate::map::{Map, MAP_HEIGHT, MAP_WIDTH};
use crate::map_builder::MapBuilder;
use crate::spawner::spawn_player;
use crate::systems::init_scheduler;

mod camera;
mod components;
mod map;
mod map_builder;
mod spawner;
mod systems;

const DISPLAY_HEIGHT: i32 = MAP_HEIGHT / 2;
const DISPLAY_WIDTH: i32 = MAP_WIDTH / 2;

const MAP_LAYER: usize = 0;
const ENTITY_LAYEAR: usize = 1;

struct State {
    ecs: World,
    resources: Resources,
    systems: Schedule,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();

        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);

        let mut resources = Resources::default();
        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));

        spawn_player(&mut ecs, map_builder.player_start);

        Self {
            ecs,
            resources,
            systems: init_scheduler(),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();

        ctx.set_active_console(1);
        ctx.cls();

        self.resources.insert(ctx.key);
        self.systems.execute(&mut self.ecs, &mut self.resources);

        render_draw_buffer(ctx).expect("failed to render draw buffer");
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
