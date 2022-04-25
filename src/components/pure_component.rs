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