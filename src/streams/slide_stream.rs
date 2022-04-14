use std::{ops::{AddAssign, Add, Sub, Mul, Div}};
use crate::streams::stream::Stream;
/**
 * Stores previous few values too, can be used for averaging or integrating.
 */
pub struct SlideStream<S: Stream, T: AddAssign + Add + Sub + Mul + Div + Copy> {
    pub parent: S,
    data: Vec<T>,
}

impl<S: Stream, T: AddAssign + Add + Sub + Mul + Div + Copy> Stream for SlideStream<S, T> where S: Stream<T = T> {
    type T = T;
    type Out = T;

    fn next(&mut self) -> T {
        let append_value = self.parent.next();
        self.data.drain(0..1);
        self.data.push(append_value);
        return append_value;
    }
}

impl<S: Stream, T: AddAssign + Add + Sub + Mul + Div + Copy> SlideStream<S, T> where S: Stream<T = T> {
    pub fn new(parent: S, size: usize, zero_value: T) -> Self {
        SlideStream { parent, data: vec![zero_value; size] }
    }

    pub fn get_data(&self) -> &Vec<T> {
        &self.data
    }

    pub fn get_size(&self) -> usize {
        self.data.len()
    }
}