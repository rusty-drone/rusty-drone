use rust_architecture::{events::{event_handler::EventHandler, impulse_event::ImpulseEvent}, streams::{constant_stream::ConstantStream, stream::Stream}, components::{pure_component::PureComponent, component::Component}, tasks::{finite_task::FiniteTask, task::Task, sequential_task::SequentialTask, parallel_task::ParallelTask}};
use core::time;
use std::{time::{Instant, Duration}, cell::RefCell, rc::Rc, thread::{sleep_ms, sleep}};
extern crate num_traits;

fn main() {

    let initial = Instant::now();

    let drivetrain = Rc::new(RefCell::new(PureComponent::new(
        Box::new(ConstantStream::<(f64, f64)>::new((0.0, 0.0))),
        Box::new(ConstantStream::<(f64, f64)>::new((12.0, 12.0))),
        |x| {println!("{} V, {} V", x.0, x.1)})));
    
    let drivetrain_c1 = drivetrain.clone();
    let drivetrain_c2 = drivetrain.clone();
    let drivetrain_c3 = drivetrain.clone();
    let drivetrain_c4 = drivetrain.clone();

    let task = FiniteTask::new(
        Box::new(move || {(*drivetrain_c1.borrow_mut()).set_controller(Box::new(ConstantStream::<(f64, f64)>::new((12.0, 12.0))));}),
        Box::new(move || {(*drivetrain_c2.borrow_mut()).reset_to_default();}),
        Box::new(move || {Instant::now().duration_since(initial).as_millis() > 3000}),
    );

    let task2 = FiniteTask::new(
        Box::new(move || {(*drivetrain_c3.borrow_mut()).set_controller(Box::new(ConstantStream::<(f64, f64)>::new((6.0, 6.0))));}),
        Box::new(move || {(*drivetrain_c4.borrow_mut()).reset_to_default();}),
        Box::new(move || {Instant::now().duration_since(initial).as_millis() > 6000}),
    );

    let sequential = SequentialTask::new(Box::new(task), Box::new(task2));

    // let actions = vec![Box::new(task)as Box<dyn Task>, Box::new(task2) as Box<dyn Task>];
    let actions = vec![Box::new(sequential)as Box<dyn Task>];

    let imp2 = ImpulseEvent::new(move | | Instant::now().duration_since(initial).as_millis() > 1000, actions);

    let mut handler = EventHandler::new(time::Duration::from_millis(200));
    handler.add_event(Box::new(imp2));

    loop {
        handler.update_events();
         (*drivetrain.borrow_mut()).apply_signal();

         sleep(Duration::from_millis(200));
    }
}