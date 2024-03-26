use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};

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
    target_render_duration: Duration,
    target_fixed_duration: Duration,
    pub(crate) min_tick_duration: Duration,
    last_render_frame_start: Instant,
    last_tick_start: Instant,
    last_fixed_tick_start: Instant,
}

impl FrameLimiter {
    pub fn new(config: FrameLimiterConfig) -> FrameLimiter {
        let target_frame_duration = {
            match config.strategy {
                FrameLimiterStrategy::Unlimited => Duration::from_secs(0),
                FrameLimiterStrategy::Sleep => {
                    let fps = config.fps.unwrap_or(60);
                    assert!(fps > 0, "FrameLimiter::config parameter `fps` is {}. This parameter must be greater than zero!", fps);
                    Duration::from_secs(1)/ fps
                }
            }
        };

        Self {
            strategy: config.strategy,
            target_render_duration: target_frame_duration,
            target_fixed_duration:  Duration::from_secs(1) / 60,
            min_tick_duration:  Duration::from_secs(1) / 60,
            last_render_frame_start: Instant::now(),
            last_fixed_tick_start: Instant::now(),
            last_tick_start: Instant::now(),
        }
    }

    pub fn render(&mut self) {
        self.last_render_frame_start = Instant::now();
    }
    pub fn fixed_tick(&mut self) {
        self.last_fixed_tick_start = Instant::now();
    }
    pub fn tick(&mut self, instant: &Instant) {
        self.last_tick_start = instant.clone();
    }

    pub fn render_unlocked(&mut self) -> bool {
        match self.strategy {
            FrameLimiterStrategy::Unlimited => {
                true
            }
            FrameLimiterStrategy::Sleep => {
                let frame_start = self.last_render_frame_start;
                let target_frame_duration = self.target_render_duration;
                let elapsed = Instant::now() - frame_start;
                if elapsed < target_frame_duration {
                    false
                } else {
                    true
                }
            }
        }
    }

    pub fn is_fixed_update(&mut self) -> bool {
        let frame_start = self.last_fixed_tick_start;
        let target_frame_duration = self.target_fixed_duration;
        let elapsed = Instant::now() - frame_start;
        if elapsed < target_frame_duration {
            false
        } else {
            true
        }
    }

    pub fn is_min_tick(&mut self) -> bool {
        let frame_start = self.last_tick_start;
        let target_frame_duration = self.min_tick_duration;
        let elapsed = Instant::now() - frame_start;
        //info!("target {:?} / elapsed {:?}", target_frame_duration, elapsed);
        if elapsed < target_frame_duration {
            false
        } else {
            true
        }
    }
}