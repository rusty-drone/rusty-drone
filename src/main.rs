// use std::borrow::{BorrowMut, Borrow};
// use std::rc::Rc;

use rust_architecture::streams::{Stream, CustomStream, SlideStream};
use rust_architecture::controllers::{derivative_controller};

fn main() {
    let mut x = 0.0;


    let stream = CustomStream::new(||  {
        println!("Accepted new gyro values");
        x += 2.0;
        x
    }).map(|x| x * 2.0);

    let s = SlideStream::new(stream, 4, 0.0);

    let mut derivative = derivative_controller(s, 1.5);

    println!("{:?}", derivative.next());
    println!("{:?}", derivative.next());
    println!("{:?}", derivative.next());
    println!("{:?}", derivative.next());
}
