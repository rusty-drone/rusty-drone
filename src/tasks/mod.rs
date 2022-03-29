pub trait Task { 
    fn initialize(&self);
    fn execute(&self);
    fn end(&self) -> bool;
}

pub struct InfiniteTask<F: Fn()> {
    f: F,
    name: String,
}

impl <F: Fn()> InfiniteTask<F> {
    pub fn new(f: F, name: String) -> Self {
        InfiniteTask { f , name }
    }
}

impl <F: Fn()> Task for InfiniteTask<F> {
    fn initialize(&self) {
        println!("Initializing {}...", self.name);
    }

    fn execute(&self) {
        (self.f)();
    }

    fn end(&self) -> bool{
        println!("Ending {}...", self.name);
        false
    }
}

pub struct FiniteTask<F: Fn(), P: Fn() -> bool> {
    name: String,
    f: F,
    conclude: P,
}

impl <F: Fn(), P: Fn() -> bool> FiniteTask<F, P> {
    pub fn new(f: F, name: String, conclude: P) -> Self {
        FiniteTask { f , name, conclude }
    }
}

impl <F: Fn(), P: Fn() -> bool> Task for FiniteTask<F, P> {
    fn initialize(&self) {
        println!("Initializing {}...", self.name);
    }

    fn execute(&self) {
        (self.f)();
    }

    fn end(&self) -> bool{
        println!("Ending {}...", self.name);
        return (self.conclude)();
    }
}

pub struct TaskHandler {
    tasks: Vec<Box<dyn Task>>,
}

impl TaskHandler {
    pub fn new() -> Self {
        TaskHandler { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, task: Box<dyn Task>) {
        task.initialize();
        self.tasks.push(task);
    }

    pub fn run(&mut self) {
        for task in self.tasks.iter_mut() {
            task.execute();
        }

        self.tasks.retain(|task| {
            !task.end()
        });
    }

    pub fn shut_down(&mut self) {
        for task in self.tasks.iter_mut() {
            task.end();
        }
    }
}