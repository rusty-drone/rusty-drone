pub trait Task { 
    fn initialize(&mut self);
    fn execute(&mut self);
    fn end(&mut self);
    fn has_finished(&mut self) -> bool;
}

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
        println!("Initializing...");
    }

    fn execute(&mut self) {
        (self.f)();
    }

    fn has_finished(&mut self) -> bool{
        false
    }

    fn end(&mut self) {
    }
}

#[derive(Clone)]
pub struct FiniteTask<F: FnMut(), P: FnMut(), E: FnMut() -> bool> {
    pub f: F,
    pub conclude: P,
    pub has_finished: E,
}

impl <F: FnMut(), P: FnMut(), E: FnMut() -> bool> FiniteTask<F, P, E> {
    pub fn new(f: F, conclude: P, has_finished: E) -> Self {
        FiniteTask { f, conclude, has_finished }
    }
}

impl <F: FnMut(), P: FnMut(), E: FnMut() -> bool> Task for FiniteTask<F, P, E> {
    fn initialize(&mut self) {
        println!("Initializing...");
    }

    fn execute(&mut self) {
        (self.f)();
    }

    fn has_finished(&mut self) -> bool{
        return (self.has_finished)();
    }

    fn end(&mut self){
        println!("Ending...");
        (self.conclude)();
    }
}

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

        // TODO: this is totally broken lol... 
        // after task is removed index is ahead by one
        // need to modify this
        for i in 0..self.tasks.len() {
            let b = &mut self.tasks[i];
            if b.has_finished() {
                b.end();
                self.tasks.remove(i);
            }
        }

        for task in self.tasks.iter_mut() {
            // initialize task
            task.execute();
        }

    //     // self.tasks.retain_mut(|task| {
    //     //     // get the task's end function
    //     //     !task.end()
    //     // });
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