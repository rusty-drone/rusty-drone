use rust_architecture::{events::{event_handler::EventHandler, impulse_event::ImpulseEvent, continuous_event::ContinuousEvent}, tasks::finite_task::FiniteTask, streams::{constant_stream::ConstantStream, stream::Stream}, components::{pure_component::PureComponent, component::Component}};
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

    let s1 = ConstantStream::<f64>::new(5.0);
    let s2 = s1.map(|x| {2.0 * x});
    let s3 = s1.map(|x| {3.0 * x});

    let mut comp = PureComponent::new(Box::new(s2), Box::new(s3), |x| {println!("Yo + {}", x)});
    comp.reset_to_default();

    let s4 = s1.map(|x| {x*x});

    comp.set_controller(Box::new(s4));

    loop {
        handler.update_events();
        comp.apply_signal();

        if Instant::now().duration_since(initial).as_millis() > 5000 {
            handler.shut_down();
            break;
        }
    }

}