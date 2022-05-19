pub mod impulse_event;
pub mod continuous_event;
pub mod event_handler;

pub trait Event {
    fn fired(&mut self) -> bool;
    fn all_tasks_finished(&mut self) -> bool;
}