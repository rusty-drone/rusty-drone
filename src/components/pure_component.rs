// use std::ops::{AddAssign, Add, Sub, Mul, Div};

// use crate::streams::stream::Stream;

// use super::component::Component;

// pub struct PureComponent<O, P, F: FnMut(O)> where O: AddAssign + Add + Sub + Mul + Div + Copy, P: AddAssign + Add + Sub + Mul + Div + Copy, S: Stream<T = O> {
//     default_controller: Box<dyn Stream<T = O, Out = P>>,
//     controller: dyn Stream<T = O, Out = P>,
//     is_default: bool,
//     f: F,
// }

// // impl <O, P, F: FnMut(O)> PureComponent<O, P, F> where O: AddAssign + Add + Sub + Mul + Div + Copy, P: AddAssign + Add + Sub + Mul + Div + Copy, S: Stream<T = O> {
// //     pub fn new(default_controller: Box<dyn Stream<T = O, Out = P>, f: F) -> Self {
// //         PureComponent {
// //             default_controller,
// //             controller: default_controller.clone(),
// //             is_default: true,
// //             f
// //         }
// //     }
// // }

// impl <O: AddAssign + Add + Sub + Mul + Div + Copy, S: Stream<T = O>, F: FnMut(O)> Component<O, S> for PureComponent<O, S, F> where S: Copy {
//     fn set_controller(&mut self, controller: S) {
//         self.controller = controller;
//         self.is_default = false;
//     }

//     fn reset_to_default(&mut self) {
//         self.is_default = true;
//     }

//     fn apply_signal(&mut self) {
//         if self.is_default {
//             (self.f)(self.default_controller.next());
//         } else {
//             (self.f)(self.controller.next());
//         }
//     }
// }

use crate::streams::stream::{Stream};

// current issue: controller cannot be any type since `S` is immediatly constrained to default controller.
// adding other generic arguments won't help since they too will be immediatly constrained to what was provided
pub struct PureComponent<S: Stream, F: FnMut()> {
    default_controller: Box<S>,
    controller: Box<S>,
    is_default: bool,
    f: F,
}