use crate::streams::stream::Stream;
use crate::streams::stream::StreamOps;


/**
 * returns only one value, can be used for PID gains or other constants
 */
#[derive(Copy, Clone)]
pub struct ConstantStream<T: StreamOps> {
   pub value: T,
}

impl<T: StreamOps> ConstantStream<T> {
    pub fn new(value: T) -> Self {
        ConstantStream { value }
    }
}

impl<T: StreamOps> Stream for ConstantStream<T> {
    type T = T;
    type Out = T;

    fn next(&mut self) -> T {
        self.value
    }
}