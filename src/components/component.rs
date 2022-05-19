use crate::streams::stream::{Stream, StreamOps};

/*
1. add clocks
*/
pub trait Component<In: StreamOps, O: StreamOps> {
    fn set_controller(&mut self, controller: Box<dyn Stream<T = In, Out = O>>);

    fn reset_to_default(&mut self);

    fn apply_signal(&mut self);
}