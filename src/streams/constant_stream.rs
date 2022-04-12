use std::{ops::{AddAssign, Add, Sub, Mul, Div}};
use crate::streams::stream::Stream;
#[derive(Copy, Clone)]

/**
 * returns only one value, can be used for PID gains or other constants
 */
pub struct ConstantStream<T: AddAssign + Add + Sub + Mul + Div + Copy> {
   pub value: T,
}

impl<T: AddAssign + Add + Sub + Mul + Div + Copy> ConstantStream<T> {
    pub fn new(value: T) -> Self {
        ConstantStream { value }
    }
}

impl<T: Copy + AddAssign + Sub + Mul + Add + Div> Stream for ConstantStream<T> {
    type T = T;
    type Out = T;

    fn next(&mut self) -> T {
        self.value
    }
}