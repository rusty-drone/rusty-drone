use rust_architecture::{tasks::{task_handler::TaskHandler}, events::{event_handler::EventHandler, continuous_event::ContinuousEvent, impulse_event::ImpulseEvent}};
use core::time;

fn main() {

    let mut handler = EventHandler::new(time::Duration::from_millis(1000));

    let time = std::time::Instant::now();

    let mut e = Box::new(ContinuousEvent::new(|| {
        println!("Event fired!");
    }));

    let i = ImpulseEvent::new(
        move || { std::time::Instant::now().duration_since(time) > time::Duration::from_millis(2000) }, || {println!("Fired!")});

    e.attach(Box::new(i));

    handler.add_event(e);

    loop {
        handler.update_events();
        if std::time::Instant::now().duration_since(time) > time::Duration::from_millis(5000) {
            handler.shut_down();
            break;
        }
    }

}