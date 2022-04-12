use std::{ops::{AddAssign, Add, Sub, Mul, Div}};
use crate::streams::constant_stream::ConstantStream;
use crate::streams::map_stream::MapStream;
use crate::streams::zip_stream::ZipStream;

/**
 * Generic implementation of `Stream`. Used for all input and output
 * data sources.
 */
pub trait Stream : Clone {
    type T: AddAssign + Add + Sub + Mul + Div + Copy;
    type Out: AddAssign + Add + Sub + Mul + Div + Copy;

    fn next(&mut self) -> Self::T;

    fn constant<T: AddAssign + Add + Sub + Mul + Div + Copy>(value: Self::T) -> ConstantStream<Self::T> {
        ConstantStream { value }
    }

    fn map<'a, Out: AddAssign + Add + Sub + Mul + Div + Copy, F: FnMut(Self::T) -> Out>(self, f: F) -> MapStream<Self, Out, F> where Self: Sized,
    {
        MapStream { parent: self, f }
    }

    fn zip<Out: AddAssign + Add + Sub + Mul + Div + Copy, S: Stream, F: FnMut(Self::T, S::T) -> Out>(self, other: S, f: F) -> ZipStream<Self, S, Out, F> where Self: Sized,
    {
        ZipStream { s: self, p: other, f }
    }
}