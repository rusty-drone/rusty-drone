// use std::ops::{AddAssign, Add, Sub, Mul, Div};

// use crate::streams::stream::Stream;

// #[derive(Debug, Copy, Clone)]
// pub struct TimeStamped<T, O> where T: AddAssign + Add + Sub + Mul + Div, O: AddAssign + Add + Sub + Mul + Div {
//     pub value: T,
//     pub time: O, //seconds
// }

// impl<T, O> TimeStamped<T, O>  where T: AddAssign + Add + Sub + Mul + Div, O: AddAssign + Add + Sub + Mul + Div {
//     pub fn new(value: T, time: O) -> TimeStamped<T, O> {
//         TimeStamped {value, time}
//     }
// }

// impl<T, O> AddAssign for TimeStamped<T, O>  where T: AddAssign + Add + Sub + Mul + Div, O: AddAssign + Add + Sub + Mul + Div{
//     fn add_assign(&mut self, rhs: Self) {
//         self.value += rhs.value;
//         self.time += rhs.time; //todo remove?
//     }
// }

// impl<T, O> Add for TimeStamped<T, O>  where T: AddAssign + Add + Sub + Mul + Div, O: AddAssign + Add + Sub + Mul + Div{
//     type Output = <T as Add>::Output;

//     fn add(self, rhs: Self) -> Self::Output {
//         self.value + rhs.value
//     }
// }

// impl<T, O> Sub for TimeStamped<T, O>  where T: AddAssign + Add + Sub + Mul + Div, O: AddAssign + Add + Sub + Mul + Div{
//     type Output = <T as Sub>::Output;

//     fn sub(self, rhs: Self) -> Self::Output {
//         return self.value - rhs.value
//     }
// }

// impl<T, O> Mul for TimeStamped<T, O>  where T: AddAssign + Add + Sub + Mul + Div, O: AddAssign + Add + Sub + Mul + Div{
//     type Output = <T as Mul>::Output;

//     fn mul(self, rhs: Self) -> Self::Output {
//         return self.value * rhs.value
//     }
// }

// impl <T, O> Mul<T> for TimeStamped<T, O>  where T: AddAssign + Add + Sub + Mul + Div, O: AddAssign + Add + Sub + Mul + Div{
//     type Output = <T as Mul<T>>::Output;

//     fn mul(self, rhs: T) -> Self::Output {
//         return self.value * rhs
//     }
// }

// impl<T, O> Div for TimeStamped<T, O>  where T: AddAssign + Add + Sub + Mul + Div, O: AddAssign + Add + Sub + Mul + Div{
//     type Output = <T as Div>::Output;

//     fn div(self, rhs: Self) -> Self::Output {
//         return self.value / rhs.value
//     }
// }

// pub fn zip_with_time<T, O, S: Stream<T = O>, D: Stream<T = T>>(time: S, data: D) -> impl Stream<T = TimeStamped<T, O>>  where T: AddAssign + Add + Sub + Mul + Div + Copy, O: AddAssign + Add + Sub + Mul + Div + Copy{
//     //returns a zip stream that zips a timestamped
//     data.zip(time, |x: D::T, y: S::T| TimeStamped{value: x, time: y})
// }