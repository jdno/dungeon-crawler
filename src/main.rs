use bracket_lib::prelude::*;
use legion::{Resources, Schedule, World};

use crate::camera::Camera;
use crate::map::{Map, MAP_HEIGHT, MAP_WIDTH};
use crate::map_builder::MapBuilder;
use crate::spawner::{spawn_amulet_of_yala, spawn_monster, spawn_player};
use crate::systems::{
    build_input_scheduler, build_monster_turn_scheduler, build_player_turn_scheduler,
};
use crate::turn_state::TurnState;

mod camera;
mod components;
mod map;
mod map_builder;
mod spawner;
mod systems;
mod turn_state;

const DISPLAY_HEIGHT: i32 = MAP_HEIGHT / 2;
const DISPLAY_WIDTH: i32 = MAP_WIDTH / 2;
const HUD_HEIGHT: i32 = MAP_HEIGHT * 2;
const HUD_WIDTH: i32 = MAP_WIDTH * 2;

const MAP_LAYER: usize = 0;
const ENTITY_LAYER: usize = 1;
const HUD_LAYER: usize = 2;

struct State {
    ecs: World,
    resources: Resources,
    input_systems: Schedule,
    monster_systems: Schedule,
    player_systems: Schedule,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();

        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);

        let mut resources = Resources::default();
        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));
        resources.insert(TurnState::AwaitingInput);

        spawn_amulet_of_yala(&mut ecs, map_builder.amulet_position);
        spawn_player(&mut ecs, map_builder.player_start);
        spawn_monsters(&mut ecs, &mut rng, map_builder.rooms);

        Self {
            ecs,
            resources,
            input_systems: build_input_scheduler(),
            monster_systems: build_monster_turn_scheduler(),
            player_systems: build_player_turn_scheduler(),
        }
    }

    fn game_over(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(HUD_LAYER);

        ctx.print_color_centered(2, RED, BLACK, "Your quest has ended.");
        ctx.print_color_centered(
            4,
            WHITE,
            BLACK,
            "Slain by a monster, your hero's journey has come to a premature end",
        );
        ctx.print_color_centered(
            5,
            WHITE,
            BLACK,
            "The Amulet of Yala remains unclaimed, and your home town is not saved.",
        );
        ctx.print_color_centered(
            8,
            YELLOW,
            BLACK,
            "Don't worry, you can always try again with a new hero.",
        );
        ctx.print_color_centered(9, GREEN, BLACK, "Press SPACE to play again.");

        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.ecs = World::default();

            let mut rng = RandomNumberGenerator::new();
            let map_builder = MapBuilder::new(&mut rng);

            self.resources = Resources::default();
            self.resources.insert(map_builder.map);
            self.resources.insert(Camera::new(map_builder.player_start));
            self.resources.insert(TurnState::AwaitingInput);

            spawn_amulet_of_yala(&mut self.ecs, map_builder.amulet_position);
            spawn_player(&mut self.ecs, map_builder.player_start);
            spawn_monsters(&mut self.ecs, &mut rng, map_builder.rooms);
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(MAP_LAYER);
        ctx.cls();

        ctx.set_active_console(ENTITY_LAYER);
        ctx.cls();

        ctx.set_active_console(HUD_LAYER);
        ctx.cls();

        self.resources.insert(ctx.key);

        ctx.set_active_console(MAP_LAYER);
        self.resources.insert(Point::from_tuple(ctx.mouse_pos()));

        let turn_state = *self.resources.get::<TurnState>().unwrap();

        match turn_state {
            TurnState::AwaitingInput => self
                .input_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::PlayerTurn => self
                .player_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::MonsterTurn => self
                .monster_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::GameOver => self.game_over(ctx),
        };

        render_draw_buffer(ctx).expect("failed to render draw buffer");
    }
}

fn spawn_monsters(ecs: &mut World, rng: &mut RandomNumberGenerator, rooms: Vec<Rect>) {
    rooms
        .iter()
        .skip(1)
        .map(|room| room.center())
        .for_each(|position| spawn_monster(ecs, rng, position));
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_font("terminal8x8.png", 8, 8)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(HUD_WIDTH, HUD_HEIGHT, "terminal8x8.png")
        .build()?;

    main_loop(context, State::new())
}
