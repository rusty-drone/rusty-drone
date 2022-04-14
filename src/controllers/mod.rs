use std::{ops::{AddAssign, Add, Sub, Mul, Div}, cell::RefCell, rc::Rc};
use crate::{streams::{stream::Stream, slide_stream::SlideStream, map_stream::MapStream, constant_stream::ConstantStream, zip_stream::ZipStream}, time::{zip_with_time, TimeStamped}};

pub fn error<O: AddAssign + Add + Sub + Mul + Div + Copy, S: Stream<T = O>, P: Stream<T = O>> (s: S, p: P) -> 
impl Stream<T = <O as Sub>::Output>
where <O as Sub>::Output: AddAssign + Copy + Sub + Mul + Div + Add{
    s.zip(p, |x: S::T, y: P::T| {y - x})
}

pub fn proportional_controller<O: AddAssign + Add + Sub + Mul + Div + Copy, S: Stream<T = O>> (s: S, p: O) -> impl Stream
where <O as Mul>::Output: AddAssign + Copy + Sub + Mul + Div + Add{
    s.map(move |x: S::T| {x * p})
}


pub fn derivative_controller<O: AddAssign + Add + Sub + Mul + Div + Copy, T: AddAssign + Add + Sub + Mul + Div + Copy, S: Stream<T = O>, P: Stream<T = T>> (s: S, time: P, d: O) -> impl Stream
where  O: Sub<Output = O> + Div<Output = O> + Copy, T: Sub<Output = O>, <O as Mul>::Output: AddAssign + Add + Sub + Mul + Div + Copy{
    // let slide = SlideStream::new(s, 2, s.next());
    let mut zip= Rc::new(RefCell::new(zip_with_time::<T, O, P, S>(time, s)));
    let s = Rc::new(RefCell::new(SlideStream::new(*zip.borrow_mut(), 2, (*zip.borrow_mut()).next())));

    let clone_1 = Rc::clone(&s);

    let c = Rc::clone(&s);
    let out = (*clone_1.borrow_mut()).map(move |next| {
        let borrow = &*c.borrow_mut();
        let change_in_data = next.value - borrow.get_data()[borrow.get_size() - 2].value;
        let change_in_time = next.time - borrow.get_data()[borrow.get_size() - 2].time;

        let derivative = change_in_data / change_in_time;   

        let d_val = d * derivative;

        TimeStamped {
            time: next.time,
            value: d_val
        }
    }
    );
    out
}   