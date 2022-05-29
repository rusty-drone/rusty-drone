use crate::tasks::task::Task;

use super::{sequential_task::SequentialTask, parallel_task::ParallelTask};

/**
 * Task that is initialized, and repeated executes until it has 
 * finished, at which point it is ended
 */

 /*
 1. add callback to prelude
 2. don't need `f` since tasks only assign streams
 */
#[derive(Clone)]
pub struct FiniteTask<P: FnMut(), C: FnMut(), E: FnMut() -> bool> {
    pub on_start: P,
    pub on_finish: C,
    pub has_finished: E,
    initialized: bool,
}

impl <P: FnMut(), C: FnMut(), E: FnMut() -> bool> FiniteTask<P, C, E> {
    pub fn new(on_start: P, on_finish: C, has_finished: E) -> Self {
        FiniteTask {on_start: on_start, on_finish: on_finish, has_finished, initialized: false}
    }
}

impl <P: FnMut(), C: FnMut(), E: FnMut() -> bool> Task for FiniteTask<P, C, E> {
    fn initialize(&mut self) {
        if !self.initialized {
            (self.on_start)();
            self.initialized = true;
        }
    }

    fn has_finished(&mut self) -> bool{
        return (self.has_finished)();
    }

    fn end(&mut self){
        (self.on_finish)();
    }
}

impl <P: FnMut(), C: FnMut(), E: FnMut() -> bool> FiniteTask<P, C, E> where P: 'static, C: 'static, E: 'static{

    pub fn then(self: Self, task: Box<dyn Task>) -> SequentialTask{
        SequentialTask::new(Box::new(self), task)
    }

    pub fn with(self: Self, task: Box<dyn Task>) -> ParallelTask {
        ParallelTask::new(Box::new(self), task)
    }
}
