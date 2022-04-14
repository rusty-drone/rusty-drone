use crate::streams::stream::StreamOps;
use crate::streams::stream::Stream;

/**
 * Maps a `Stream` through a function.
 */
#[derive(Copy, Clone)]
pub struct MapStream<P: Stream, Out: StreamOps, F: FnMut(P::T) -> Out> {
    pub parent: P,
    pub f: F,
}

impl<P: Stream, Out: StreamOps, F: FnMut(P::T) -> Out> Stream for MapStream<P, Out, F>{
    type T = Out;
    type Out = Out;

    fn next(&mut self) -> Out {
        (self.f)(self.parent.next())
    }
}