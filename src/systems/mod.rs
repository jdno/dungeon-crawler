use legion::Schedule;

mod input;
mod map_render;

pub fn init_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(input::process_input_system())
        .add_system(map_render::render_map_system())
        .build()
}
