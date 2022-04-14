use rust_architecture::{events::{event_handler::EventHandler}};
use core::time;
extern crate num_traits;

fn main() {

    let mut handler = EventHandler::new(time::Duration::from_millis(200));

    loop {
        handler.update_events();
    }

}
