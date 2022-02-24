use legion::Schedule;

pub fn init_scheduler() -> Schedule {
    Schedule::builder().build()
}
