use rust_architecture::{events::{event_handler::EventHandler, impulse_event::ImpulseEvent}, streams::{constant_stream::ConstantStream, stream::Stream}, components::{pure_component::PureComponent, component::Component}, tasks::{finite_task::FiniteTask, task::Task}};
use core::time;
use std::{time::{Instant, Duration}, cell::RefCell, rc::Rc, thread::{sleep_ms, sleep}};
extern crate num_traits;

// fn main() {

//     let mut handler = EventHandler::new(time::Duration::from_millis(200));

//     let initial = Instant::now();

//     let s1 = ConstantStream::<f64>::new(12.0);
//     let s2 = s1.map(|x| {2.0 * x});
//     let s3 = s1.map(|x| {3.0 * x});

//     let comp= Rc::new(RefCell::new(PureComponent::new(Box::new(s2), Box::new(s3), |x| {println!("{}", x)})));

//     let c = comp.clone();
//     let c2 = comp.clone();

//     let actions_2 = vec![Box::new( 
//         move || {(*c.borrow_mut()).reset_to_default();},
//     )as Box<dyn FnMut()>, Box::new( || {println!("2nd action");} )as Box<dyn FnMut()>];

//     let actions_3 = vec![Box::new( 
//         move || {(*c2.borrow_mut()).set_controller(Box::new(s1));},
//     )as Box<dyn FnMut()>];

//     let imp2 = ImpulseEvent::new(move | | Instant::now().duration_since(initial).as_millis() > 3000, actions_2);
//     let imp3 = ImpulseEvent::new(move | | Instant::now().duration_since(initial).as_millis() > 2000, actions_3);

//     handler.add_event(Box::new(imp2));
//     handler.add_event(Box::new(imp3));

//     loop {
//         handler.update_events();
//         (*comp.borrow_mut()).apply_signal();

//         if Instant::now().duration_since(initial).as_millis() > 5000 {
//             handler.shut_down();
//             break;
//         }
//     }
// }

fn main() {

    let initial = Instant::now();

    let drivetrain = Rc::new(RefCell::new(PureComponent::new(
        Box::new(ConstantStream::<(f64, f64)>::new((0.0, 0.0))),
        Box::new(ConstantStream::<(f64, f64)>::new((12.0, 12.0))),
        |x| {println!("{}, {}", x.0, x.1)})));
    
    let drivetrain_c1 = drivetrain.clone();
    let drivetrain_c2 = drivetrain.clone();

    let mut task = FiniteTask::new(
        Box::new(move || {(*drivetrain_c1.borrow_mut()).set_controller(Box::new(ConstantStream::<(f64, f64)>::new((12.0, 12.0))));}),
        Box::new(move || {(*drivetrain_c2.borrow_mut()).reset_to_default();}),
        Box::new(move || {Instant::now().duration_since(initial).as_millis() > 3000}),
    );

    let mut task2 = FiniteTask::new(
        Box::new(move || {println!("2nd action");}),
        Box::new(move || {println!("3rd action");}),
        Box::new(move || {Instant::now().duration_since(initial).as_millis() > 5000}),
    );

    let actions = vec![Box::new(task)as Box<dyn Task>, Box::new(task2) as Box<dyn Task>];

    let imp2 = ImpulseEvent::new(move | | Instant::now().duration_since(initial).as_millis() > 1000, actions);

    // task.initialize(); // will later be part of an event

    let mut prev_finish = false;

    let mut handler = EventHandler::new(time::Duration::from_millis(200));
    handler.add_event(Box::new(imp2));

    loop {
        handler.update_events();
         (*drivetrain.borrow_mut()).apply_signal();

         sleep(Duration::from_millis(200));
    }
}