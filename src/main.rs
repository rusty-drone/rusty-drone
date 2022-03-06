use rust_architecture::streams::{self, Stream};

fn main() {
    println!("Hello, world!");
    let mut x = streams::ConstantStream::new(1).map(|x| x + 1);
    std::println!("{:?}", x.next());
    std::println!("{:?}", x.next());
    std::println!("{:?}", x.next());
    std::println!("{:?}", x.next());
}
