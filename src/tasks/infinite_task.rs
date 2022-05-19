use crate::tasks::task::Task;
/**
 * continuously running task.
 */
pub struct InfiniteTask<F: FnMut()> {
    f: F,
}

impl <F: FnMut()> InfiniteTask<F> {
    pub fn new(f: F) -> Self {
        InfiniteTask { f }
    }
}

impl <F: FnMut()> Task for InfiniteTask<F>{
    fn initialize(&mut self) {
    }

    fn has_finished(&mut self) -> bool{
        false
    }

    fn end(&mut self) {
    }
}