use rust_architecture::tasks::{task_handler::TaskHandler};
use core::time;

fn main() {

    let mut handler = TaskHandler::new(time::Duration::from_millis(1000));

    let time = std::time::Instant::now();

    loop {
        handler.run();
        if std::time::Instant::now().duration_since(time) > time::Duration::from_millis(5000) {
            handler.shut_down();
            break;
        }
    }

}