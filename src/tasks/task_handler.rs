// use std::{time::Duration, thread};

// use crate::tasks::task::Task;
// pub struct TaskHandler {
//     pub tasks: Vec<Box<dyn Task>>,
//     delay: Duration,
//     elapsed_time: Duration,
//     num_loops: u128,
// }

// impl TaskHandler {
//     pub fn new(delay: Duration) -> Self {
//         TaskHandler { tasks: Vec::new(), delay, elapsed_time: Duration::from_millis(0), num_loops: 0 }
//     }

//     pub fn add_task(&mut self, mut task: Box<dyn Task>){
//         task.initialize();
//         self.tasks.push(task);
//     }

//     pub fn run(&mut self) {

//         let start_time = std::time::Instant::now();

//         let mut idx = 0 as usize;

//         while  idx < self.tasks.len() {
//             let b = &mut self.tasks[idx];
//             if b.has_finished() {
//                 b.end();
//                 self.tasks.remove(idx);
//                 continue;
//             }
//             idx = idx + 1;
//         }

//         for task in self.tasks.iter_mut() {
//             task.execute();
//         }

//         thread::sleep(self.delay);

//         let end_time = std::time::Instant::now();

//         let elapsed = end_time.duration_since(start_time);

//         self.elapsed_time += elapsed;
//         self.num_loops += 1;
//     }

//     pub fn shut_down(&mut self) {
//         println!("Shutting down...");
//         for i in 0..self.tasks.len() {
//             let b = &mut self.tasks[i];
//             b.end();
//             self.tasks.remove(i);
//         }

//         println!("Average loop time: {}", self.elapsed_time.as_millis() / self.num_loops);
//     }
// }