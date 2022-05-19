use crate::{events::Event, tasks::task::Task};

/**
 * Used to initialize more events. Is a "one time" event with no running phase.
 */

 /*
 1.rename fire to condition ✓
 2.support different actions ✓
 3.don't use event loop, use "interrupts"
 */
pub struct ImpulseEvent<F: FnMut() -> bool> {
    condition: F,
    action: Vec<Box<dyn Task>>,
}

impl<F: FnMut() -> bool> ImpulseEvent<F> {
    pub fn new(condition: F, action: Vec<Box<dyn Task>>) -> Self{
        ImpulseEvent{ condition, action}
    }
}

impl<F: FnMut() -> bool> Event for ImpulseEvent<F> {
    fn fired(&mut self) -> bool {
        let res = (self.condition)();
        if res {
            for i in 0..self.action.len() {
                (self.action[i]).initialize();
            }
        }
        res
    }

    fn all_tasks_finished(&mut self) -> bool {
        let mut idx = 0 as usize;

        while  idx < self.action.len() {
            if self.action[idx].has_finished() {
                self.action[idx].end();
                self.action.remove(idx);
                continue;
            }
            idx = idx + 1;
        }

        self.action.len() == 0
    }
}