use crate::events::Event;
pub struct ContinuousEvent<A: FnMut()> {
    listeners: Vec<Box<dyn Event>>,
    action: A,
}

impl<A: FnMut()> ContinuousEvent<A> {
    pub fn new(action: A) -> Self {
        ContinuousEvent { listeners: vec![], action }
    }

    pub fn attach(&mut self, event: Box<dyn Event>) {
        self.listeners.push(event);
    }
}

impl<A: FnMut()> Event for ContinuousEvent<A> {
    fn fired(&mut self) -> bool{

        let mut fired = false;
        
        let mut idx = 0 as usize;

        while  idx < self.listeners.len() {
            let e = &mut self.listeners[idx];
            if e.fired() {
                (self.action)();
                self.listeners.remove(idx);
                fired = true;
                continue;
            }
            idx = idx + 1;
        }
        return fired;
    }
}