use rust_architecture::streams::{self, Stream};
use rust_architecture::controllers::{self, error, proportional_controller};

fn main() {
    println!("Hello, world!");
    let p = 0.04;
    let mut y = streams::CustomStream::new(|| {
        let gyro_x = 15.0;
        println!("Recieved gyro data");
        gyro_x
    });
    let mut gyro_p = proportional_controller(y, p);
    std::println!("{:?}", gyro_p.next());
    std::println!("{:?}", gyro_p.next());

    //motor.SetDutyCycle(y.next());
}
