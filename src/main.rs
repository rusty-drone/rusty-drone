use rust_architecture::{events::{event_handler::EventHandler, impulse_event::ImpulseEvent}, streams::{constant_stream::ConstantStream, stream::Stream}, components::{pure_component::PureComponent, component::Component}};
use core::time;
use std::{time::Instant, cell::RefCell, rc::Rc};
extern crate num_traits;

fn main() {

    let mut handler = EventHandler::new(time::Duration::from_millis(200));

    let initial = Instant::now();

    let s1 = ConstantStream::<f64>::new(12.0);
    let s2 = s1.map(|x| {2.0 * x});
    let s3 = s1.map(|x| {3.0 * x});

    let comp= Rc::new(RefCell::new(PureComponent::new(Box::new(s2), Box::new(s3), |x| {println!("{}", x)})));

    let c = comp.clone();
    let c2 = comp.clone();
    let imp2 = ImpulseEvent::new(move | | Instant::now().duration_since(initial).as_millis() > 2000, move || {(*c.borrow_mut()).reset_to_default();});
    let imp3 = ImpulseEvent::new(move | | Instant::now().duration_since(initial).as_millis() > 3000, move || {(*c2.borrow_mut()).set_controller(Box::new(s1));});

    handler.add_event(Box::new(imp2));
    handler.add_event(Box::new(imp3));

    loop {
        handler.update_events();
        (*comp.borrow_mut()).apply_signal();

        if Instant::now().duration_since(initial).as_millis() > 5000 {
            handler.shut_down();
            break;
        }
    }
}