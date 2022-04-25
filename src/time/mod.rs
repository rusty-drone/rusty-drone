use std::ops::{AddAssign, Add, Sub, Mul, Div, RemAssign, DivAssign, MulAssign, SubAssign, Rem};

use num_traits::{Num, Zero, One};

use crate::streams::{stream::{Stream, StreamOps}, zip_stream::ZipStream};

#[derive(Debug, Copy, Clone)]
pub struct TimeStamped<T, O> where T: StreamOps, O: StreamOps {
    pub value: T,
    pub time: O, //seconds
}

impl <T, O>StreamOps for TimeStamped<T, O> where T: StreamOps, O: StreamOps { }

impl <T, O> RemAssign for TimeStamped<T, O> where T: StreamOps, O: StreamOps {
    fn rem_assign(&mut self, other: Self) {
        self.value %= other.value;
    }
}

impl <T, O> AddAssign for TimeStamped<T, O> where T: StreamOps, O: StreamOps {
    fn add_assign(&mut self, other: Self) {
        self.value += other.value;
    }
}

impl <T, O> DivAssign for TimeStamped<T, O> where T: StreamOps, O: StreamOps {
    fn div_assign(&mut self, other: Self) {
        self.value /= other.value;
    }
}

impl <T, O> MulAssign for TimeStamped<T, O> where T: StreamOps, O: StreamOps {
    fn mul_assign(&mut self, other: Self) {
        self.value *= other.value;
    }
}

impl <T, O> SubAssign for TimeStamped<T, O> where T: StreamOps, O: StreamOps {
    fn sub_assign(&mut self, other: Self) {
        self.value -= other.value;
    }
}

impl <T, O> Rem for TimeStamped<T, O> where T: StreamOps, O: StreamOps {
    type Output = TimeStamped<T, O>;

    fn rem(self, rhs: Self) -> Self::Output {
        TimeStamped {
            value: self.value % rhs.value,
            time: self.time % rhs.time,
        }
    }
}
impl <T, O> Div for TimeStamped<T, O> where T: StreamOps, O: StreamOps {
    type Output = TimeStamped<T, O>;

    fn div(self, rhs: Self) -> Self::Output {
        TimeStamped {
            value: self.value / rhs.value,
            time: self.time / rhs.time,
        }
    }
}
impl <T, O> Sub for TimeStamped<T, O> where T: StreamOps, O: StreamOps {
    type Output = TimeStamped<T, O>;

    fn sub(self, rhs: Self) -> Self::Output {
        TimeStamped {
            value: self.value - rhs.value,
            time: self.time - rhs.time,
        }
    }
}
impl <T, O> Mul for TimeStamped<T, O> where T: StreamOps, O: StreamOps {
    type Output = TimeStamped<T, O>;

    fn mul(self, rhs: Self) -> Self::Output {
        TimeStamped {
            value: self.value * rhs.value,
            time: self.time * rhs.time,
        }
    }
}
impl <T, O> Add for TimeStamped<T, O> where T: StreamOps, O: StreamOps {
    type Output = TimeStamped<T, O>;

    fn add(self, rhs: Self) -> Self::Output {
        TimeStamped {
            value: self.value + rhs.value,
            time: self.time + rhs.time,
        }
    }
}
impl <T, O> Zero for TimeStamped<T, O> where T: StreamOps, O: StreamOps {
    fn zero() -> Self {
        TimeStamped { value: T::zero(), time: O::zero() }
    }

    fn is_zero(&self) -> bool {
        self.value.is_zero() && self.time.is_zero()
    }
}
impl <T, O> One for TimeStamped<T, O> where T: StreamOps, O: StreamOps {
    fn one() -> Self {
        TimeStamped { value: T::one(), time: O::one() }
    }
}
impl <T, O> PartialEq for TimeStamped<T, O> where T: StreamOps, O: StreamOps {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.time == other.time
    }
}

impl <T, O> Num for TimeStamped<T, O> where T: StreamOps, O: StreamOps {
    type FromStrRadixErr = T;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> { //TODO: Fix this
        <TimeStamped<T, O> as Num>::from_str_radix(str, radix)
    }
}


pub fn zip_with_time<T, D: Stream>(time: D, data: D) -> ZipStream<D, D, TimeStamped<D::T, D::T>, impl Fn(D::T, D::T) -> TimeStamped<D::T, D::T> + Copy> where T: StreamOps{
    //returns a zip stream that zips a timestamped
    let f = |x: D::T, y: D::T| {TimeStamped{value: x, time: y}};
    return ZipStream {s: data, p: time, f};
}