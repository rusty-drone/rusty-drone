use std::{ops::{AddAssign, Add, Sub, Mul, Div}};
use crate::streams::{Stream, SlideStream, CustomStream};

pub fn error<O: AddAssign + Add + Sub + Mul + Div + Copy, S: Stream<T = O>, P: Stream<T = O>> (s: S, p: P) -> 
impl Stream<T = <O as Sub>::Output>
where <O as Sub>::Output: AddAssign + Copy + Sub + Mul + Div + Add{
    s.zip(p, |x: S::T, y: P::T| {y - x})
}

pub fn proportional_controller<O: AddAssign + Add + Sub + Mul + Div + Copy, S: Stream<T = O>> (s: S, p: O) -> 
impl Stream<T = <O as Mul>::Output>
where <O as Mul>::Output: AddAssign + Copy + Sub + Mul + Div + Add{
    s.map(move |x: S::T| {x * p})
}

//TODO(#1): update this to use TimeStamped

// pub fn derivative_controller<O: AddAssign + Add + Sub + Mul + Div + Copy, S: Stream<T = O>> ( mut s: SlideStream<S, O>, d: O) -> 
// impl Stream<T = <<O as Sub>::Output as Mul>::Output> 
// where <<O as Sub>::Output as Mul>::Output: AddAssign + Copy + Sub + Mul + Div + Add,
//  <O as Sub>::Output: Mul,
//  O: Sub<Output = O>{
//     if s.get_size() <= 1 {
//         panic!("SlideStream window is not of compatible size for derivative operation!");
//     }

//     return CustomStream::new( move || {
//         // s.next() is placed at size - 1, so the previous value is at size - 2. This is why the window must be greater than 1
//         (s.next() - s.get_data()[s.get_size() - 2]) * d
//     });
// }
