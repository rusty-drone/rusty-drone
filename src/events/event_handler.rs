use std::{time::Duration, thread};

use super::Event;

/**
 * Main event loop manager. Used to add tasks, update all tasks, etc.
 * also records loop times for debugging.
 */
pub struct EventHandler {
    listeners: Vec<Box<dyn Event>>,
    delay: Duration,
    elapsed_time: Duration,
    num_loops: u128
}

impl EventHandler {
    pub fn new(delay: Duration) -> Self {
        EventHandler { listeners: Vec::new(), delay, elapsed_time: Duration::from_millis(0), num_loops: 0 }
    }

    pub fn add_event(&mut self, event: Box<dyn Event>) {
        self.listeners.push(event);
    }

    pub fn update_events(&mut self) {
        let start_time = std::time::Instant::now();

        let mut idx = 0 as usize;

        while  idx < self.listeners.len() {
            let b = &mut self.listeners[idx];
            if b.fired() {
                self.listeners.remove(idx);
                println!("Removed index {} from listeners", idx);
                continue;
            }
            idx = idx + 1;
        }

        thread::sleep(self.delay);


        let end_time = std::time::Instant::now();

        let elapsed = end_time.duration_since(start_time);

        self.elapsed_time += elapsed;
        self.num_loops += 1;
    }
    
    pub fn shut_down(&mut self) {
        println!("Shutting down...");

        self.listeners.retain(|_| {false});

        println!("Average loop time: {}", self.elapsed_time.as_millis() / self.num_loops);
    }
}