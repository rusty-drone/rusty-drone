use super::impulse_event::{Event};
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

    pub fn fire(&mut self) {
        
        let mut idx = 0 as usize;

        while  idx < self.listeners.len() {
            let e = &mut self.listeners[idx];
            if e.fired() {
                (self.action)();
                self.listeners.remove(idx);
                continue;
            }
            idx = idx + 1;
        }
    }
}