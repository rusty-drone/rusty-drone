use crate::streams::stream::StreamOps;
use crate::streams::stream::Stream;

/**
 * Combines two Streams with a custom function.
 */
pub struct ZipStream<S: Stream, P: Stream, Out: StreamOps, F: FnMut(S::T, P::T) -> Out> {
    pub s: S, //parent 1
    pub p: P, //parent 2
    pub f: F,
}

impl<S: Stream, P: Stream, Out: StreamOps, F: FnMut(S::T, P::T) -> Out> Stream for ZipStream<S, P, Out, F>{
    type T = Out;
    type Out = Out;

    fn next(&mut self) -> Out {
        (self.f)(self.s.next(), self.p.next())
    }
}