use rust_architecture::{events::{event_handler::EventHandler, impulse_event::ImpulseEvent, continuous_event::ContinuousEvent}, tasks::finite_task::FiniteTask};
use core::time;
use std::{time::Instant, cell::RefCell, rc::Rc};
extern crate num_traits;

fn main() {

    let mut handler = EventHandler::new(time::Duration::from_millis(200));

    let initial = Instant::now();
    let count = Rc::new(RefCell::new(0));
    let count2 = Rc::clone(&count);

    let task = FiniteTask::new(move | | {*count.borrow_mut() += 1;}, || {}, || {println!("Finished!"); }, move | | {*count2.borrow() >= 10});

    let impulse = ImpulseEvent::new(move | | Instant::now().duration_since(initial).as_millis() > 2000, || {println!("Instantiated.")});

    let mut e = ContinuousEvent::new(|| {}, Box::new(task));
    e.attach(Box::new(impulse));

    handler.add_event(Box::new(e));

    loop {
        handler.update_events();
    }

}