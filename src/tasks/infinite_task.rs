use crate::tasks::task::Task;
/**
 * continuously running task.
 */
pub struct InfiniteTask<F: FnMut()> {
    on_start: F,
    initialized: bool,
}

impl <F: FnMut()> InfiniteTask<F> {
    pub fn new(on_start: F) -> Self {
        InfiniteTask { on_start, initialized: false }
    }
}

impl <F: FnMut()> Task for InfiniteTask<F>{
    fn initialize(&mut self) {
        if !self.initialized {
            (self.on_start)();
            self.initialized = true;
        }
    }

    fn has_finished(&mut self) -> bool{
        false
    }

    fn end(&mut self) {
    }
}