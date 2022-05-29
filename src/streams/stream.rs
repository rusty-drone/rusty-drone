use crate::{streams::constant_stream::ConstantStream};
use crate::streams::map_stream::MapStream;
use crate::streams::zip_stream::ZipStream;

pub trait StreamOps: Copy {}

impl StreamOps for f32 {}
impl StreamOps for i32 {}
impl StreamOps for f64 {}
impl StreamOps for i64 {}
impl StreamOps for (f64, f64) {}
impl StreamOps for bool {}
/**
 * Generic implementation of `Stream`. Used for all input and output
 * data sources.
 */
pub trait Stream {
    type T: StreamOps;
    type Out: StreamOps;

    fn next(&mut self) -> Self::T;

    fn constant(value: Self::T) -> ConstantStream<Self::T> where Self: Sized,{
        ConstantStream { value }
    }

    fn map<O: StreamOps, F: FnMut(Self::T) -> O>(self, f: F) -> MapStream<Self, O, F> where Self: Sized,
    {
        MapStream { parent: self, f }
    }

    fn zip<S: Stream, O: StreamOps, F: FnMut(Self::T, S::T) -> O>(self, other: S, f: F) -> ZipStream<Self, S, O, F> where Self: Sized, F: Copy,
    {
        ZipStream { s: self, p: other, f }
    }
}