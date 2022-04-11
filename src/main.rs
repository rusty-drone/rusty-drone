use rust_architecture::{events::{event_handler::EventHandler, continuous_event::ContinuousEvent, impulse_event::ImpulseEvent}, tasks::{finite_task::FiniteTask, infinite_task::InfiniteTask}};
use core::time;
use std::{rc::Rc, cell::RefCell};

fn main() {

    let mut handler = EventHandler::new(time::Duration::from_millis(200));

    let time = std::time::Instant::now();

    let original = Rc::new(RefCell::new(0));
    let c1 = Rc::clone(&original);
    let c2 = Rc::clone(&original);

    let task = FiniteTask::new(
        move || {
            *c1.borrow_mut() += 1;
            println!("{}", *c1.borrow());
        },
        || {println!("Initialized Count Task.")},
        || {println!("Finished Count Task.")},
        move || {*c2.borrow_mut() >= 10}
    );

    let mut e = Box::new(ContinuousEvent::new(|| {
        println!("Event fired!");
    }, Box::new(task)));

    let i = ImpulseEvent::new(
        move || { std::time::Instant::now().duration_since(time) > time::Duration::from_millis(2000) }, || {println!("Fired!")});

    e.attach(Box::new(i));

    handler.add_event(e);

    loop {
        handler.update_events();
        if std::time::Instant::now().duration_since(time) > time::Duration::from_millis(10000) {
            handler.shut_down();
            break;
        }
    }

}
