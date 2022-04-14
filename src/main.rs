use rust_architecture::{events::{event_handler::EventHandler}, components::{component::Component}, streams::{constant_stream::ConstantStream, custom_stream::CustomStream}};
use core::time;

fn main() {

    // let mut component = PureComponent::new(ConstantStream::new(5.0), |x| println!("{}", x));

    let mut handler = EventHandler::new(time::Duration::from_millis(200));

    // component.set_controller(ConstantStream::new(10.0));

    // component.set_controller(CustomStream::new(|| {println!("Hi"); 2.0}));

    loop {
        // component.apply_signal();
        handler.update_events();
    }

}
