use rust_architecture::tasks::{TaskHandler, FiniteTask};
use core::time;
use std::{thread, rc::Rc, cell::RefCell};

fn main() {

    let original = Rc::new(RefCell::new(0));
    let c1 = Rc::clone(&original);
    let c2 = Rc::clone(&original);

    let task = FiniteTask::new(
     move || {
        *c1.borrow_mut() += 1;
        println!("{}", *c1.borrow_mut());
    }, 

    || {},
     
    move || {
        *c2.borrow_mut() >= 10
    });

    let mut handler = TaskHandler::new();

    handler.add_task(Box::new(task));

    loop {
        handler.run();
        thread::sleep(time::Duration::from_millis(200));
    }
}