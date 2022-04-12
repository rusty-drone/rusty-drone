use std::{ops::{AddAssign, Add, Sub, Mul, Div}};
use crate::streams::stream::Stream;

/**
 * Combines two Streams with a custom function.
 */
pub struct ZipStream<S: Stream, P: Stream, Out: AddAssign + Add + Sub + Mul + Div + Copy, F: FnMut(S::T, P::T) -> Out> {
    pub s: S, //parent 1
    pub p: P, //parent 2
    pub f: F,
}

impl<S: Stream, P: Stream, Out: AddAssign + Add + Sub + Mul + Div + Copy, F: FnMut(S::T, P::T) -> Out> Stream for ZipStream<S, P, Out, F>  where F: Clone{
    type T = Out;
    type Out = Out;

    fn next(&mut self) -> Out {
        (self.f)(self.s.next(), self.p.next())
    }
}

impl<S: Stream, P: Stream, Out: AddAssign + Add + Sub + Mul + Div + Copy, F: FnMut(S::T, P::T) -> Out> Clone for ZipStream<S, P, Out, F>  where F: Clone{
    fn clone(&self) -> Self {
        ZipStream { s: self.s.clone(), p: self.p.clone(), f: self.f.clone() }
    }
}