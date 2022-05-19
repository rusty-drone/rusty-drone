// use crate::{events::Event, tasks::task::Task};
// /**
//  * Has a vector of `ImpulseEvent` which start up the `task`. When fired, 
//  * if provided it executes an `action` and in the meantime continues to 
//  * manage the `task`.
//  */
// pub struct ContinuousEvent<A: FnMut()> {
//     listeners: Vec<Box<dyn Event>>,
//     action: A,
//     instantiated_task: bool,
//     task_initialized: bool,
//     has_fired: bool,
//     pub task: Box<dyn Task>,
// }

// impl<A: FnMut()> ContinuousEvent<A> {
//     pub fn new(action: A, task: Box<dyn Task>) -> Self {
//         ContinuousEvent { listeners: vec![], action, instantiated_task: false, task_initialized: false, has_fired: false, task }
//     }

//     pub fn attach(&mut self, event: Box<dyn Event>) {
//         self.listeners.push(event);
//     }
// }

// impl<A: FnMut()> Event for ContinuousEvent<A> {
//     fn fired(&mut self) -> bool{
        
//         let mut idx = 0 as usize;

//         while  idx < self.listeners.len() {
//             let e = &mut self.listeners[idx];
//             if e.fired() {
//                 self.instantiated_task = true;
//                 self.has_fired = true;
//                 (self.action)();
//                 self.listeners.remove(idx);
//                 continue;
//             }
//             idx = idx + 1;
//         }

//         if self.instantiated_task && !self.task_initialized{
//             self.task.initialize();
//             self.task_initialized = true;
//         } else if self.instantiated_task {
//             self.task.execute();
//         }

//         let finished = self.task.has_finished();
//         if finished { 
//             self.task.end();
//         }

//         return self.has_fired && finished;
//     }
// }