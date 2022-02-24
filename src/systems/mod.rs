use legion::Schedule;

mod process_input;
mod render_entity;
mod render_map;

pub fn init_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(process_input::process_input_system())
        .add_system(render_entity::render_entity_system())
        .add_system(render_map::render_map_system())
        .build()
}
