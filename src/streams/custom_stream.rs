use crate::streams::stream::StreamOps;
use super::stream::Stream;

/**
 * Custom fetch function that might use 3rd party libraries
 */
#[derive(Copy)]
pub struct CustomStream<Out: StreamOps, F: FnMut() -> Out>{
    pub fetch: F,
}

impl<Out: StreamOps, F: FnMut() -> Out> CustomStream<Out, F> {
    pub fn new(fetch: F) -> Self {
        CustomStream { fetch }
    }
}

impl<Out: StreamOps, F: FnMut() -> Out> Stream for CustomStream<Out, F> where F: Clone{
    type T = Out;
    type Out = Out;

    fn next(&mut self) -> Out {
        (self.fetch)()
    }
}

//impl clone for custom stream
impl<Out: StreamOps, F: FnMut() -> Out> Clone for CustomStream<Out, F>  where F: Clone{
    fn clone(&self) -> Self {
        CustomStream {fetch: self.fetch.clone() }
    }
}