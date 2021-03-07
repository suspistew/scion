use serde::{Deserialize, Serialize};
use std::thread;
use std::time::{Duration, Instant};

pub(crate) static mut FRAME_LOCKED: bool = false;

/// In order to reduce the cpu usage, the `FrameLimiter` will handle an
/// ecs Lock if a frame used less time than expected.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) enum FrameLimiterStrategy {
    /// The `FrameLimiter` won't try to sleep and will launch the next ecs frame
    /// immediately after the previous one.
    Unlimited,
    /// The `FrameLimiter` will compute the expected duration of a frame and
    /// do a background sleep, locking the ecs, but not the rendering or window events.
    /// usize is the maximum expected fps number
    Sleep,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct FrameLimiterConfig {
    strategy: FrameLimiterStrategy,
    fps: Option<u32>,
}

impl Default for FrameLimiterConfig {
    fn default() -> Self {
        Self {
            strategy: FrameLimiterStrategy::Sleep,
            fps: Some(60),
        }
    }
}

pub(crate) struct FrameLimiter {
    strategy: FrameLimiterStrategy,
    target_frame_duration: Duration,
    frame_start: Instant,
}

impl FrameLimiter {
    pub fn new(config: FrameLimiterConfig) -> FrameLimiter {
        let target_frame_duration = {
            match config.strategy {
                FrameLimiterStrategy::Unlimited => Duration::from_secs(0),
                FrameLimiterStrategy::Sleep => {
                    let fps = config.fps.unwrap_or(60);
                    assert!(fps > 0, "FrameLimiter::config parameter `fps` is {}. This parameter must be greater than zero!", fps);
                    Duration::from_secs(1) / fps
                }
            }
        };

        Self {
            strategy: config.strategy,
            target_frame_duration,
            frame_start: Instant::now(),
        }
    }

    pub fn start_frame(&mut self) {
        self.frame_start = Instant::now();
    }

    pub fn end_frame(&mut self) {
        match self.strategy {
            FrameLimiterStrategy::Unlimited => {
                // Just continue
            }
            FrameLimiterStrategy::Sleep => {
                let frame_start = self.frame_start;
                let target_frame_duration = self.target_frame_duration;
                unsafe {
                    FRAME_LOCKED = true;
                };
                thread::spawn(move || {
                    let elapsed = Instant::now() - frame_start;
                    if elapsed < target_frame_duration {
                        spin_sleep::sleep(target_frame_duration - elapsed);
                    }
                    unsafe {
                        FRAME_LOCKED = false;
                    }
                });
            }
        }
    }
}
