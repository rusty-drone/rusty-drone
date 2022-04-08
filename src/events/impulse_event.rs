pub trait Event {
    fn fired(&mut self) -> bool;
}

/**
 * Used to initialize more events. Is a "one time" event with no running phase.
 */
pub struct ImpulseEvent<F: FnMut() -> bool, A: FnMut()> {
    fire: F,
    ended: bool,
    action: A,
}

impl<F: FnMut() -> bool, A: FnMut()> ImpulseEvent<F, A> {
    pub fn new(fire: F, action: A) -> Self{
        ImpulseEvent{ fire, ended: false, action}
    }
}

impl<F: FnMut() -> bool, A: FnMut()> Event for ImpulseEvent<F, A> {
    fn fired(&mut self) -> bool {
        let res = (self.fire)();
        if res {
            (self.action)();
            self.ended = true;
        }
        res
    }
}