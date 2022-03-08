use std::{ops::{AddAssign, Add, Sub, Mul, Div}};
pub trait Stream {
    type T: AddAssign + Add + Sub + Mul + Div + Copy;
    type Out: AddAssign + Add + Sub + Mul + Div + Copy;

    fn next(&mut self) -> Self::T;

    fn constant<T: AddAssign + Add + Sub + Mul + Div + Copy>(value: Self::T) -> ConstantStream<Self::T> {
        ConstantStream { value }
    }

    fn map<Out: AddAssign + Add + Sub + Mul + Div + Copy, F: Fn(Self::T) -> Out>(self, f: F) -> MapStream<Self, Out, F> where Self: Sized,
    {
        MapStream { parent: self, f }
    }

    fn zip<Out: AddAssign + Add + Sub + Mul + Div + Copy, S: Stream, F: Fn(Self::T, S::T) -> Out>(self, other: S, f: F) -> ZipStream<Self, S, Out, F> where Self: Sized,
    {
        ZipStream { s: self, p: other, f }
    }
}

#[derive(Copy, Clone)]
pub struct ConstantStream<T: AddAssign + Add + Sub + Mul + Div + Copy> {
    value: T,
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

#[derive(Copy, Clone)]
pub struct MapStream<P: Stream, Out: AddAssign + Add + Sub + Mul + Div + Copy, F: Fn(P::T) -> Out> {
    parent: P,
    f: F,
}

impl<P: Stream, Out: AddAssign + Add + Sub + Mul + Div + Copy, F: Fn(P::T) -> Out> Stream for MapStream<P, Out, F> {
    type T = Out;
    type Out = Out;

    fn next(&mut self) -> Out {
        (self.f)(self.parent.next())
    }
}

// used to synthesize the output between to streams
#[derive(Copy, Clone)]
pub struct ZipStream<S: Stream, P: Stream, Out: AddAssign + Add + Sub + Mul + Div + Copy, F: Fn(S::T, P::T) -> Out> {
    s: S, //parent 1
    p: P, //parent 2
    f: F,
}

impl<S: Stream, P: Stream, Out: AddAssign + Add + Sub + Mul + Div + Copy, F: Fn(S::T, P::T) -> Out> Stream for ZipStream<S, P, Out, F> {
    type T = Out;
    type Out = Out;

    fn next(&mut self) -> Out {
        (self.f)(self.s.next(), self.p.next())
    }
}

#[derive(Clone)]
pub struct SlideStream<S: Stream, T: AddAssign + Add + Sub + Mul + Div + Copy> {
    parent: S,
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

// used for sensor readings or other 3rd party data
#[derive(Copy, Clone)]
pub struct CustomStream<Out: AddAssign + Add + Sub + Mul + Div + Copy, F: FnMut() -> Out> {
    fetch: F,
}

impl<Out: AddAssign + Add + Sub + Mul + Div + Copy, F: FnMut() -> Out> CustomStream<Out, F> {
    pub fn new(fetch: F) -> Self {
        CustomStream { fetch }
    }
}

impl<Out: AddAssign + Add + Sub + Mul + Div + Copy, F: FnMut() -> Out> Stream for CustomStream<Out, F> {
    type T = Out;
    type Out = Out;

    fn next(&mut self) -> Out {
        (self.fetch)()
    }
}
