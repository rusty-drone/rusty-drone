use std::{ops::{AddAssign, Add, Sub, Mul, Div}};
use crate::streams::stream::Stream;
/**
 * Maps a `Stream` through a function.
 */
pub struct MapStream<P: Stream, Out: AddAssign + Add + Sub + Mul + Div + Copy, F: Fn(P::T) -> Out> {
    pub parent: P,
    pub f: F,
}

impl<P: Stream, Out: AddAssign + Add + Sub + Mul + Div + Copy, F: Fn(P::T) -> Out> Clone for MapStream<P, Out, F>  where F: Clone{
    fn clone(&self) -> Self {
        MapStream { parent: self.parent.clone(), f: self.f.clone() }
    }
}

impl<P: Stream, Out: AddAssign + Add + Sub + Mul + Div + Copy, F: Fn(P::T) -> Out> Stream for MapStream<P, Out, F>  where F: Clone{
    type T = Out;
    type Out = Out;

    fn next(&mut self) -> Out {
        (self.f)(self.parent.next())
    }
}