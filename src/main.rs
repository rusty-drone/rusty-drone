// use core::time;
// use std::cell::RefCell;
// use std::thread;
// use std::time::{Duration};

// use rust_architecture::streams::{CustomStream};
// use rust_architecture::tasks::{TaskHandler, InfiniteTask};

// fn main() {
//     let mut count = 0;

//     // let mut handler = TaskHandler::new();


//     let mut task = InfiniteTask::new(
//         || {
//             println!("executing...");
//             count += 1;
//         });

//     let mut s = CustomStream::new(|| {
//         // count += 1;
//         // count
//         1
//     });

//     // let mut task = FiniteTask::new(
//     //     move || {
//     //         let count = count.as_ref();
//     //         println!("{}", count)
//     //     },
//     //  "finite task test".to_string(),
//     //   || {println!("COUNTED!")},
//     //    move || 
//     //    {
//     //        count.as_ref() > &10
//     //     }
//     // );
//     let temp = RefCell::new(task);
//     handler.add_task(temp);

//     let mut duration = Duration::new(0, 0);

//     loop {
//         handler.run();
//         thread::sleep(time::Duration::from_millis(2000));
//         duration += time::Duration::from_millis(2000);

//         if duration == time::Duration::from_secs(10) {
//             handler.shut_down();
//         }

//     }
// }

// // fn main() {
// //     let initial = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
// //         Ok(n) => n.as_nanos(),
// //         Err(_) => 0,
// //     };

// //     let mut data = CustomStream::new(|| {
// //         let t = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
// //             Ok(n) => n.as_nanos(),
// //             Err(_) => 0,
// //         };
// //         t - initial
// //     });    

// //     let time = CustomStream::new(|| {
// //         let t = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
// //             Ok(n) => n.as_nanos(),
// //             Err(_) => 0,
// //         };
// //         t - initial
// //     });

// //     let mut zip = zip_with_time(time, data);
// //     let mut slide = SlideStream::new(zip.clone(), 4, zip.next());

// //     std::println!("{:?}", data.next());
// //     std::println!("{:?}", data.next());
// //     std::println!("{:?}", slide.next());
// //     std::println!("{:?}", slide.next());

// //     println!("{:?}", slide.get_data());

// //     let fetch = |next: TimeStamped<u128, u128>| {
// //         let change_in_data = (next.value - slide.get_data()[slide.get_size() - 2].value) % 31;
// //         let change_in_time = (next.time - slide.get_data()[slide.get_size() - 2].time) % 31;
        
// //         change_in_data / change_in_time
// //     };

// //     let mut d = slide.clone().map(&fetch);

    

// //     println!("{:?}", d.next());
// //     println!("{:?}", d.parent.get_data());
// //     println!("{:?}", d.next());
// //     println!("{:?}", d.parent.get_data());
// //     println!("{:?}", d.next());
// //     println!("{:?}", d.parent.get_data());
// //     println!("{:?}", d.next());
// //     println!("{:?}", d.parent.get_data());
// //     println!("{:?}", d.next());
// //     println!("{:?}", d.parent.get_data());
// //     println!("{:?}", d.next());
// //     println!("{:?}", d.parent.get_data());
// //     println!("{:?}", d.next());
// //     println!("{:?}", d.parent.get_data());
// //     println!("{:?}", d.next());
// //     println!("{:?}", d.parent.get_data());

// //     // let mut derivative = derivative_controller(data, time, 2);
// //     // println!("{:?}", derivative.next());
// //     // println!("{:?}", derivative.next());
// //     // println!("{:?}", derivative.next());
// // }

use rust_architecture::tasks::{TaskHandler, InfiniteTask};
use core::time;
use std::{thread};

fn main() {

    let mut count = 0;

    let task = InfiniteTask::new(
    move || {
        println!("e1...");
        count += 1;
        println!("{}", count);
    });

    let task2 = InfiniteTask::new(
        move || {
            println!("e2...");
            count += 1;
            println!("{}", count);
        });

    let task3 = InfiniteTask::new(
        move || {
            println!("e3...");
            count += 1;
            println!("{}", count);
        });

    let task4 = InfiniteTask::new(
        move || {
            println!("e4...");
            count += 1;
            println!("{}", count);
        });

    let mut handler = TaskHandler::new();

    handler.add_task(Box::new(task));
    handler.add_task(Box::new(task2));
    handler.add_task(Box::new(task3));
    handler.add_task(Box::new(task4));

    loop {
        handler.run();
        thread::sleep(time::Duration::from_millis(200));
    }
}