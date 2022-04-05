use rust_architecture::tasks::task_handler::TaskHandler;
use rust_architecture::tasks::finite_task::FiniteTask;
use rust_architecture::tasks::sequential_task::SequentialTask;
use core::time;
use std::{thread, rc::Rc, cell::RefCell};

use rust_architecture::streams::constant_stream::ConstantStream;

fn main() {

    let s = ConstantStream::new(5);

    let original = Rc::new(RefCell::new(0));
    let c1 = Rc::clone(&original);
    let c2 = Rc::clone(&original);

    let o1 = Rc::clone(&original);
    let o2 = Rc::clone(&original);

    let task = FiniteTask::new(
     move || {
        *c1.borrow_mut() += 1;
    }, 

    || {},

    || {
        println!("Task 1 completed");
    },
     
    move || {
        *c2.borrow_mut() >= 10
    });

    let task2 = FiniteTask::new(
        move || {
           *o1.borrow_mut() += 1;
           println!("{}", s.value);
       }, 
   
       || {},
   
       || {
           println!("Task 2 completed");
       },
        
       move || {
           *o2.borrow_mut() >= 30
       });

    let s = SequentialTask::new(Box::new(task), Box::new(task2));

    let mut handler = TaskHandler::new();

    handler.add_task(Box::new(s));

    loop {
        handler.run();
        thread::sleep(time::Duration::from_millis(200));
    }
}