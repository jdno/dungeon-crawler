use bracket_lib::prelude::*;
use legion::world::SubWorld;
use legion::{component, system, IntoQuery};

use crate::components::{Health, Player};
use crate::map::NUM_TILES;
use crate::{HUD_LAYER, HUD_WIDTH};

#[system]
#[read_component(Health)]
#[read_component(Player)]
pub fn render_hud(ecs: &SubWorld) {
    let player_health = <&Health>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .next()
        .unwrap();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(HUD_LAYER);

    draw_batch.print_centered(1, "Explore the Dungeon. Cursor keys to move.");
    draw_batch.bar_horizontal(
        Point::zero(),
        HUD_WIDTH,
        player_health.current,
        player_health.max,
        ColorPair::new(RED, BLACK),
    );
    draw_batch.print_color_centered(
        0,
        format!(" Health: {} / {}", player_health.current, player_health.max),
        ColorPair::new(WHITE, RED),
    );

    draw_batch
        .submit(NUM_TILES + 5000)
        .expect("failed to submit draw batch");
}
