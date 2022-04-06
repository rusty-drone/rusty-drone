use rust_architecture::{tasks::{task_handler::TaskHandler}};
use core::time;

fn main() {

    let mut handler = TaskHandler::new(time::Duration::from_millis(1000));

    let time = std::time::Instant::now();

    // let s = ConstantStream::new(5);
    // let time = CustomStream::new(|| {
    //     let t = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
    //         Ok(t) => t,
    //         Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    //     };
    //     std::thread::sleep(time::Duration::from_millis(100));
    //     t.as_millis()
    // });
    // let mut d = derivative_controller(s, time, 1);
    // let v = d.next();
    // println!("{:?}", d.next());
    // println!("{:?}", d.next());
    // println!("{:?}", d.next());
    // println!("{:?}", d.next());

    loop {
        handler.run();
        if std::time::Instant::now().duration_since(time) > time::Duration::from_millis(5000) {
            handler.shut_down();
            break;
        }
    }

}