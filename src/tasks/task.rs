pub trait Task { 
    fn initialize(&mut self);
    fn execute(&mut self);
    fn end(&mut self);
    fn has_finished(&mut self) -> bool;
}
