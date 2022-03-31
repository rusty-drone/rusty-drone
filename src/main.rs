use core::time;
use std::borrow::BorrowMut;
use std::rc::Rc;
use std::thread;
use std::time::{Duration, SystemTime};

use rust_architecture::streams::{Stream, CustomStream, SlideStream, ConstantStream};
// use rust_architecture::controllers::{proportional_controller, derivative_controller};
use rust_architecture::time::{zip_with_time, TimeStamped};

use rust_architecture::tasks::{FiniteTask, InfiniteTask, Task, TaskHandler};

fn main() {
    let count = Rc::new(0);
    let mut handler = TaskHandler::new();


    let mut task = InfiniteTask::new(
        || {
            println!("executing...")
        }, "infinite task test".to_string());

    // let mut task = FiniteTask::new(
    //     move || {
    //         let count = count.as_ref();
    //         println!("{}", count)
    //     },
    //  "finite task test".to_string(),
    //   || {println!("COUNTED!")},
    //    move || 
    //    {
    //        count.as_ref() > &10
    //     }
    // );
    handler.add_task(Box::new(task));

    let mut duration = Duration::new(0, 0);

    loop {
        handler.run();
        thread::sleep(time::Duration::from_millis(2000));
        duration += time::Duration::from_millis(2000);

        if duration == time::Duration::from_secs(10) {
            handler.shut_down();
        }

    }
}

// fn main() {
//     let initial = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
//         Ok(n) => n.as_nanos(),
//         Err(_) => 0,
//     };

//     let mut data = CustomStream::new(|| {
//         let t = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
//             Ok(n) => n.as_nanos(),
//             Err(_) => 0,
//         };
//         t - initial
//     });    

//     let time = CustomStream::new(|| {
//         let t = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
//             Ok(n) => n.as_nanos(),
//             Err(_) => 0,
//         };
//         t - initial
//     });

//     let mut zip = zip_with_time(time, data);
//     let mut slide = SlideStream::new(zip.clone(), 4, zip.next());

//     std::println!("{:?}", data.next());
//     std::println!("{:?}", data.next());
//     std::println!("{:?}", slide.next());
//     std::println!("{:?}", slide.next());

//     println!("{:?}", slide.get_data());

//     let fetch = |next: TimeStamped<u128, u128>| {
//         let change_in_data = (next.value - slide.get_data()[slide.get_size() - 2].value) % 31;
//         let change_in_time = (next.time - slide.get_data()[slide.get_size() - 2].time) % 31;
        
//         change_in_data / change_in_time
//     };

//     let mut d = slide.clone().map(&fetch);

    

//     println!("{:?}", d.next());
//     println!("{:?}", d.parent.get_data());
//     println!("{:?}", d.next());
//     println!("{:?}", d.parent.get_data());
//     println!("{:?}", d.next());
//     println!("{:?}", d.parent.get_data());
//     println!("{:?}", d.next());
//     println!("{:?}", d.parent.get_data());
//     println!("{:?}", d.next());
//     println!("{:?}", d.parent.get_data());
//     println!("{:?}", d.next());
//     println!("{:?}", d.parent.get_data());
//     println!("{:?}", d.next());
//     println!("{:?}", d.parent.get_data());
//     println!("{:?}", d.next());
//     println!("{:?}", d.parent.get_data());

//     // let mut derivative = derivative_controller(data, time, 2);
//     // println!("{:?}", derivative.next());
//     // println!("{:?}", derivative.next());
//     // println!("{:?}", derivative.next());
// }
