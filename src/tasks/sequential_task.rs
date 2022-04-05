use crate::tasks::task::Task;

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

impl Task for  SequentialTask{
    fn initialize(&mut self) {
        self.first.initialize();
    }

    fn execute(&mut self) {
        if !self.first.has_finished() {
            self.first.execute();
        }

        else if !self.second_phase {
            self.first.end();
            self.second.initialize();
            self.second_phase = true;
        }

        if self.second_phase {
            self.second.execute();
        }
    }

    fn end(&mut self) {
        self.second.end();
    }

    fn has_finished(&mut self) -> bool {
        self.second.has_finished()
    }
}