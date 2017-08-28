use std::time::SystemTime;
use std::collections::VecDeque;

const BUFFER_SIZE: u32 = 16;

/// Keeps track of current average framerate
pub struct FpsCounter {
    current_time: SystemTime,
    previous_times: VecDeque<u32>,
}

impl FpsCounter {
    /// Creates and returns a new instance of teh FpsCounter struct
    pub fn new() -> FpsCounter {
        FpsCounter {
            current_time: SystemTime::now(),
            previous_times: VecDeque::<u32>::with_capacity(BUFFER_SIZE as usize),
        }
    }

    /// Triggered each frame to record the time since the previous
    pub fn tick(&mut self) {
        if let Ok(elapsed) = self.current_time.elapsed() {
            self.previous_times.push_back(elapsed.subsec_nanos());
            if self.previous_times.len() as u32 > BUFFER_SIZE {
                self.previous_times.pop_front();
            }
        }
        self.current_time = SystemTime::now();
    }

    /// Returns the current framerate
    pub fn framerate(&self) -> u32 {
        if !self.previous_times.is_empty() {
            let average = self.previous_times.iter().sum::<u32>() / BUFFER_SIZE as u32;
            1_000_000_000 / average
        } else {
            0u32
        }
    }
}
