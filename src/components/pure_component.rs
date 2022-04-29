use crate::streams::stream::{Stream, StreamOps};

use super::component::Component;

// // current issue: controller cannot be any type since `S` is immediatly constrained to default controller.
// // adding other generic arguments won't help since they too will be immediatly constrained to what was provided
pub struct PureComponent<F: FnMut(In), In: StreamOps, O: StreamOps> {
    default_controller: Box<dyn Stream<T = In, Out = O>>,
    controller: Box<dyn Stream<T = In, Out = O>>,
    is_default: bool,
    f: F,
}

impl<F: FnMut(In), In: StreamOps, O: StreamOps> PureComponent<F, In, O> {
    pub fn new(default_controller: Box<dyn Stream<T = In, Out = O>>, controller: Box<dyn Stream<T = In, Out = O>>, f: F) -> PureComponent<F, In, O> {
        PureComponent {
            default_controller,
            controller: controller,
            is_default: true,
            f
        }
    }
}


impl<F: FnMut(In), In: StreamOps, O: StreamOps> Component<In, O> for PureComponent<F, In, O> {
    fn set_controller(&mut self, controller: Box<dyn Stream<T = In, Out = O>>) {
        self.controller = controller;
        self.is_default = false;
    }

    fn reset_to_default(&mut self) {
        self.is_default = true;
    }

    fn apply_signal(&mut self) {
        if self.is_default {
            (self.f)(self.default_controller.next());
        } else {
            (self.f)(self.controller.next());
        }
    }
}
