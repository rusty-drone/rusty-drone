use std::ops::{AddAssign, Add, Sub, Mul, Div};

use crate::streams::stream::Stream;

pub trait Component<O, S: Stream<T = O>> where O: AddAssign + Add + Sub + Mul + Div + Copy {
    fn set_controller(&mut self, controller: S);

    fn reset_to_default(&mut self);

    fn apply_signal(&mut self);
}