use legion::Schedule;

mod chase_player;
mod combat;
mod end_turn;
mod fov;
mod move_entity;
mod move_randomly;
mod process_input;
mod render_entity;
mod render_hud;
mod render_map;
mod render_tooltip;

pub fn build_input_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(process_input::process_input_system())
        .add_system(fov::field_of_view_system())
        .flush()
        .add_system(render_entity::render_entity_system())
        .add_system(render_map::render_map_system())
        .add_system(render_hud::render_hud_system())
        .add_system(render_tooltip::render_tooltip_system())
        .build()
}

pub fn build_monster_turn_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(move_randomly::move_randomly_system())
        .add_system(chase_player::chase_player_system())
        .flush()
        .add_system(combat::combat_system())
        .flush()
        .add_system(move_entity::move_entity_system())
        .flush()
        .add_system(fov::field_of_view_system())
        .flush()
        .add_system(render_entity::render_entity_system())
        .add_system(render_map::render_map_system())
        .add_system(render_hud::render_hud_system())
        .add_system(end_turn::end_turn_system())
        .build()
}

pub fn build_player_turn_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(combat::combat_system())
        .flush()
        .add_system(move_entity::move_entity_system())
        .flush()
        .add_system(fov::field_of_view_system())
        .flush()
        .add_system(render_entity::render_entity_system())
        .add_system(render_map::render_map_system())
        .add_system(render_hud::render_hud_system())
        .add_system(end_turn::end_turn_system())
        .build()
}
