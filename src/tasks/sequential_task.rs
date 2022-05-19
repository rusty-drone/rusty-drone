use crate::tasks::task::Task;

/**
 * Once first task is finished, it's ended and second task has begun. 
 * Runs tasks one after another.
 */
pub struct SequentialTask{
    pub first: Box<dyn Task>,
    pub second: Box<dyn Task>,
    first_phase: bool,
    second_phase: bool
}

impl SequentialTask {
    pub fn new(first: Box<dyn Task>, second: Box<dyn Task>) -> Self {
        SequentialTask { first, second, first_phase: true, second_phase: false }
    }
}

impl Task for SequentialTask{
    fn initialize(&mut self) {
        if self.first.has_finished() && !self.second_phase {
            self.second_phase = true;
            self.first_phase = false;
            self.first.end();
        }

        if self.first_phase {
            self.first.initialize();
            self.first_phase = false;
        }
        else if self.second_phase {
            self.second.initialize();
        }
    }

    fn end(&mut self) {
        self.second.end();
    }

    fn has_finished(&mut self) -> bool {
        self.second.has_finished() && self.second_phase
    }
}