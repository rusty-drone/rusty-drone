use crate::tasks::task::Task;

/**
 * Two tasks that are run simultaneously.
 */
pub struct ParallelTask {
    pub first: Box<dyn Task>,
    pub second: Box<dyn Task>,
    first_finished: bool,
    second_finished: bool,
    first_init: bool,
    second_init: bool,
}

impl ParallelTask {
    pub fn new(first: Box<dyn Task>, second: Box<dyn Task>) -> ParallelTask{
        ParallelTask { first, second, first_finished: false, second_finished: false, first_init: false, second_init: false }
    }
}

impl Task for ParallelTask {
    fn initialize(&mut self) {
        if !self.first_init {
            self.first.initialize();
            self.first_init = true;
        }
        if !self.second_init {
            self.second.initialize();
            self.second_init = true;
        }

        if self.first.has_finished() && !self.first_finished {
            self.first_finished = true;
            self.first.end();
        }

        if self.second.has_finished() && !self.second_finished {
            self.second_finished = true;
            self.second.end();
        }
    }

    fn end(&mut self) {
    }

    fn has_finished(&mut self) -> bool {
        self.first_finished && self.second_finished
    }
}