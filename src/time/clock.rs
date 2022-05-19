pub trait Clock {
    fn apply(&self, f: &mut dyn FnMut());
}