// // use std::{ops::{AddAssign, Add, Sub, Mul, Div}};
// // use crate::{streams::{stream::Stream, slide_stream::SlideStream}, time::{zip_with_time, TimeStamped}};

// // pub fn error<O: AddAssign + Add + Sub + Mul + Div + Copy, S: Stream<T = O>, P: Stream<T = O>> (s: S, p: P) -> 
// // impl Stream<T = <O as Sub>::Output>
// // where <O as Sub>::Output: AddAssign + Copy + Sub + Mul + Div + Add{
// //     s.zip(p, |x: S::T, y: P::T| {y - x})
// // }

// // pub fn proportional_controller<O: AddAssign + Add + Sub + Mul + Div + Copy, S: Stream<T = O>> (s: S, p: O) -> 
// // impl Stream<T = <O as Mul>::Output>
// // where <O as Mul>::Output: AddAssign + Copy + Sub + Mul + Div + Add{
// //     s.map(move |x: S::T| {x * p})
// // }

// // pub fn derivative_controller<O: AddAssign + Add + Sub + Mul + Div + Copy, T: AddAssign + Add + Sub + Mul + Div + Copy, S: Stream<T = O>, P: Stream<T = T>> (s: S, time: P, d: O) -> impl Stream<T = TimeStamped<<O as Mul<O>>::Output, T>>
// // where  O: Sub<Output = O> + Div<Output = O> + Copy, T: Sub<Output = O>, <O as Mul>::Output: AddAssign + Add + Sub + Mul + Div + Copy{
// //     // let slide = SlideStream::new(s, 2, s.next());
// //     let mut zip = zip_with_time(time, s);
// //     let s = SlideStream::new(zip.clone(), 2, zip.next());

// //     let clone = s.clone();

// //     let out = s.map(move |next| {
    
// //         let change_in_data = next.value - clone.get_data()[clone.get_size() - 2].value;
// //         let change_in_time = next.time - clone.get_data()[clone.get_size() - 2].time;

// //         let derivative = change_in_data / change_in_time;   

// //         let d_val = d * derivative;

// //         TimeStamped {
// //             time: next.time,
// //             value: d_val
// //         }
// //     }
// //     );
// //     out
// // }   

// use std::{cell::RefCell, rc::Rc};

// use crate::{streams::{stream::{StreamOps, Stream}, slide_stream::SlideStream}, time::{zip_with_time, TimeStamped}};

// pub fn error<O: StreamOps, S: Stream>(s: S, s2: S) -> impl Stream{
//     s.zip(s2, |x: S::T, y: S::T| {y - x})
// }

// pub fn proportional<O: StreamOps, S: Stream<T = O>>(s: S, p: O) -> impl Stream{
//     s.map(move |x: S::T| {x * p})
// }

// pub fn derivative<O: StreamOps, S: Stream<T = O>>(s: S, time: S, d: O) -> impl Stream{
//     let mut zip = zip_with_time::<O, S>(time, s);
//     let mut slide = SlideStream::new(zip, 2, zip.next());
// }   