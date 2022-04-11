use crate::tasks::task::Task;

/**
 * Task that is initialized, and repeated executes until it has 
 * finished, at which point it is ended
 */
#[derive(Clone)]
pub struct FiniteTask<F: FnMut(), P: FnMut(), C: FnMut(), E: FnMut() -> bool> {
    pub f: F,
    pub prelude: P,
    pub conclude: C,
    pub has_finished: E,
}

impl <F: FnMut(), P: FnMut(), C: FnMut(), E: FnMut() -> bool> FiniteTask<F, P, C, E> {
    pub fn new(f: F, prelude: P, conclude: C, has_finished: E) -> Self {
        FiniteTask { f, prelude, conclude, has_finished}
    }

    //TODO: implement `then` to be able to easily generate sequential tasks
    // pub fn then(&self, task: Box<dyn Task>) -> SequentialTask{
    //     SequentialTask::new(Box::new(&self as &dyn Task), task)
    // }
}

impl <F: FnMut(), P: FnMut(), C: FnMut(), E: FnMut() -> bool> Task for FiniteTask<F, P, C, E> {
    fn initialize(&mut self) {
        (self.prelude)();
    }

    fn execute(&mut self) {
        (self.f)();
    }

    fn has_finished(&mut self) -> bool{
        return (self.has_finished)();
    }

    fn end(&mut self){
        (self.conclude)();
    }
}