use std::{ops::{AddAssign, Add, Sub, Mul, Div}};
use crate::{streams::{Stream}};

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
//`TimeStamped<T, O>`: `<Output = TimeStamped<T, O>>`
// pub fn derivative_controller<O: AddAssign + Add + Sub + Mul + Div + Copy, T: AddAssign + Add + Sub + Mul + Div + Copy, S: Stream<T = T>, P: Stream<T = O>> ( data: S, time: P, d: T) -> impl Stream<T = TimeStamped<<T as Mul>::Output, O>> 
// where <<T as Mul>::Output as Div>::Output : AddAssign + Add + Sub + Mul + Div + Copy, <T as Sub>::Output: AddAssign + Add + Sub + Mul + Div + Copy, T: Sub<Output = T>, O: Sub<Output = T>, <T as Mul>::Output: AddAssign + Add + Sub + Mul + Div + Copy {
//     let mut zip = zip_with_time(time, data);
//     let slide = SlideStream::new(zip.clone(), 2, zip.next());
//     let clone = slide.clone();
//     let out = slide.map(move |next| {
        
//         let change_in_data = next.value - clone.get_data()[clone.get_size() - 2].value;
//         let change_in_time = next.time - clone.get_data()[clone.get_size() - 2].time;
        
//         let o = d * change_in_data / change_in_time;

//         TimeStamped{value: o * d, time: next.time}
//     }
//     );
//     return out;
// }
