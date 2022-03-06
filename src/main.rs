use rust_architecture::streams::{self, Stream};
use rust_architecture::controllers::{self, error, proportional_controller};
fn main() {
    println!("Hello, world!");
    let mut x = streams::ConstantStream::new(1).map(|x| x + 1);
    let mut y = streams::ConstantStream::new(8);
    let mut z = error(x, y);

    let mut prop = proportional_controller(z, 2);
    std::println!("{:?}", prop.next());
}
