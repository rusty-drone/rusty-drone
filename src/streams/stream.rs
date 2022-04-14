use std::{ops::{AddAssign, Add, Sub, Mul, Div}};
use crate::streams::constant_stream::ConstantStream;
use crate::streams::map_stream::MapStream;
use crate::streams::zip_stream::ZipStream;

/**
 * Generic implementation of `Stream`. Used for all input and output
 * data sources.
 */
pub trait Stream : Sized {
    type T: AddAssign + Add + Sub + Mul + Div + Copy;
    type Out: AddAssign + Add + Sub + Mul + Div + Copy;

    fn next(&mut self) -> Self::T;

    fn constant(value: Self::T) -> ConstantStream<Self::T> {
        ConstantStream { value }
    }

    fn map<O: AddAssign + Add + Sub + Mul + Div + Copy, F: FnMut(Self::T) -> O>(self, f: F) -> MapStream<Self, O, F> where Self: Sized,
    {
        MapStream { parent: self, f }
    }

    fn zip<S: Stream, O: AddAssign + Add + Sub + Mul + Div + Copy, F: FnMut(Self::T, S::T) -> O>(self, other: S, f: F) -> ZipStream<Self, S, O, F> where Self: Sized,
    {
        ZipStream { s: self, p: other, f }
    }
}