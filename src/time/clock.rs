pub trait Clock {
    fn apply(&mut self);
    fn get_tick_speed(&self) -> std::time::Duration;
}