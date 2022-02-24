use legion::Schedule;

mod input;

pub fn init_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(input::input_system())
        .build()
}
