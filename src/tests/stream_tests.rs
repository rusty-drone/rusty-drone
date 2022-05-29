#[cfg(test)]
mod streams_tests{

    use std::{cell::RefCell, rc::Rc};

    use crate::streams::{constant_stream::ConstantStream, stream::{Stream}, custom_stream::CustomStream};

    #[test]
    fn constant_stream() {
        let mut s = ConstantStream::new(5.0);
        assert_eq!(s.next(), 5.0);

        let mut s = ConstantStream::<i32>::new(3);
        assert_eq!(s.next(), 3);
    }

    #[test]
    fn map_stream() {
        let c = ConstantStream::new(5.0);
        let mut s = c.map(|x| {2.0 * x});

        assert_eq!(s.next(), 10.0);

        let mut s2 = s.map(|x| {x*x});
        assert_eq!(s2.next(), 100.0);
    }

    #[test]
    fn zip_stream() {

        let c1 = Rc::new(RefCell::new(ConstantStream::<f64>::new(2.0)));
        let c2 = ConstantStream::<f64>::new(3.0);

        let mut z = (*c1.borrow_mut()).zip(c2, |x, y| {x + y});
        assert_eq!(z.next(), 5.0);

        let mut z2 = z.zip(*c1.borrow_mut(), |x, y| x * y);
        assert_eq!(z2.next(), 10.0);
    }

    #[test]
    fn custom_stream() {
        let mut c = CustomStream::new(|| {5.0}); //in this case, functions as a constant stream

        assert_eq!(c.next(), 5.0);
    }

    #[test]
    fn diff_in_outs() {
        // maps from f64 --> bool --> f32
        let s = ConstantStream::new(5.0);
        let mut o = s.map(|x| {
            if x > 4.0 {
                true
            }
            else {
                false
            }
        }).map(|x| {
            if x {
                8.0 as f32
            }
            else {
                8.0 as f32
            }
        });
        assert_eq!(o.next(), 8.0);
    }
}