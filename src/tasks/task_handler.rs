use crate::tasks::task::Task;
pub struct TaskHandler {
    pub tasks: Vec<Box<dyn Task>>,
}

impl TaskHandler {
    pub fn new() -> Self {
        TaskHandler { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, mut task: Box<dyn Task>){
        task.initialize();
        self.tasks.push(task);
    }

    pub fn run(&mut self) {

        let mut idx = 0 as usize;

        while  idx < self.tasks.len() {
            let b = &mut self.tasks[idx];
            if b.has_finished() {
                b.end();
                self.tasks.remove(idx);
                continue;
            }
            idx = idx + 1;
        }

        for task in self.tasks.iter_mut() {
            task.execute();
        }
    }

    pub fn shut_down(&mut self) {
        println!("Shutting down...");
        for i in 0..self.tasks.len() {
            let b = &mut self.tasks[i];
            b.end();
            self.tasks.remove(i);
        }
    }
}