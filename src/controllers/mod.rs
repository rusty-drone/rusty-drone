use std::{ops::{AddAssign, Add, Sub, Mul, Div}};
use crate::streams::{Stream, ZipStream};

pub fn error<O: AddAssign + Add + Sub + Mul + Div + Copy, S: Stream<T = O>, P: Stream<T = O>> (s: S, p: P) -> impl Stream<T = <O as Sub>::Output>
where <O as Sub>::Output: AddAssign + Copy + Sub + Mul + Div + Add{
    s.zip(p, |x: S::T, y: P::T| {y - x})
}


pub fn proportional_controller<O: AddAssign + Add + Sub + Mul + Div + Copy, S: Stream<T = O>> (s: S, p: O) -> impl Stream<T = <O as Mul>::Output>
where <O as Mul>::Output: AddAssign + Copy + Sub + Mul + Div + Add{
    s.map(move |x: S::T| {x * p})
}