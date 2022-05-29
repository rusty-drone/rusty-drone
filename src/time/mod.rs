pub mod clock;
pub mod clock_handler;
// use crate::streams::{stream::{Stream, StreamOps}, zip_stream::ZipStream};

// #[derive(Debug, Copy)]
// pub struct TimeStamped{
//     pub value: Box<dyn StreamOps>,
//     pub time: Box<dyn StreamOps>, //seconds
// }

// pub fn zip_with_time<T, D: Stream>(time: D, data: D){
//     //returns a zip stream that zips a timestamped
//     let f = |x: D::T, y: D::T| {TimeStamped{value: x, time: y}};
//     return ZipStream {s: data, p: time, f};
// }