use crate::tasks::task::Task;

/**
 * Once first task is finished, it's ended and second task has begun. 
 * Runs tasks one after another.
 */
pub struct SequentialTask{
    pub first: Box<dyn Task>,
    pub second: Box<dyn Task>,
    second_phase: bool
}

impl SequentialTask {
    pub fn new(first: Box<dyn Task>, second: Box<dyn Task>) -> Self {
        SequentialTask { first, second, second_phase: false }
    }
}

impl Task for SequentialTask{
    fn initialize(&mut self) {
        self.first.initialize();
    }

    fn end(&mut self) {
        self.second.end();
    }

    fn has_finished(&mut self) -> bool {
        self.second.has_finished()
    }
}