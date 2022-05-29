use crate::streams::stream::{Stream, StreamOps};

use super::component::Component;
use super::super::time::clock::Clock;

pub struct PureComponent<F: FnMut(In), In: StreamOps, O: StreamOps> {
    default_controller: Box<dyn Stream<T = In, Out = O>>,
    controller: Box<dyn Stream<T = In, Out = O>>,
    is_default: bool,
    f: F,
    tick_speed: std::time::Duration
}

impl<F: FnMut(In), In: StreamOps, O: StreamOps> PureComponent<F, In, O> {
    pub fn new(default_controller: Box<dyn Stream<T = In, Out = O>>, controller: Box<dyn Stream<T = In, Out = O>>, f: F, tick_speed: std::time::Duration) -> PureComponent<F, In, O> {
        let mut out = PureComponent {
            default_controller,
            controller,
            is_default: true,
            f,
            tick_speed
        };
        out.reset_to_default();
        out
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

impl<F: FnMut(In), In: StreamOps, O: StreamOps> Clock for PureComponent<F, In, O> {
    fn apply(&mut self) {
        self.apply_signal();
    }

    fn get_tick_speed(&self) -> std::time::Duration {
        self.tick_speed
    }
}
