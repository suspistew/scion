use std::time::{Duration, Instant};

/// ['Time'] is a resource dedicated to compute the time durations between frames and keep a track of
/// frame numbers
pub struct Time {
    delta_duration: Duration,
    frame_number: u64,
    measure_start: Instant,
}

impl Default for Time {
    fn default() -> Self {
        Self {
            delta_duration: Default::default(),
            frame_number: 0,
            measure_start: Instant::now(),
        }
    }
}

impl Time {
    pub(crate) fn frame(&mut self) {
        self.frame_number += 1;
        self.delta_duration = self.measure_start.elapsed();
        self.measure_start = Instant::now();
    }

    /// Returns the duration of the last executed frame
    pub fn delta_duration(&self) -> Duration {
        self.delta_duration
    }
}
