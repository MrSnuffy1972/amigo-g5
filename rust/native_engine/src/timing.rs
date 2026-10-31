//! The fixed-rate game clock: game logic runs at the game's own tick rate (50 Hz for a PAL game) whatever the display
//! refresh is; the platform draws the latest frame at the display's rate. After a stall of more than a few ticks the
//! clock is set again from now, so a game never runs fast to catch up.

use std::time::{Duration, Instant};

pub struct FixedStep {
    pub tick: Duration,
    base: Instant,
    base_tick: u64,
    pub done: u64,
}

/// Ticks owed beyond this are dropped (the game slows down instead of racing).
const MAX_BEHIND: u64 = 5;

impl FixedStep {
    pub fn new(hz: u32) -> FixedStep {
        FixedStep {
            tick: Duration::from_nanos(1_000_000_000 / hz.max(1) as u64),
            base: Instant::now(),
            base_tick: 0,
            done: 0,
        }
    }

    /// How many ticks to run now (`now` is the current time).
    pub fn due(&mut self, now: Instant) -> u32 {
        let owed = self.base_tick
            + (now.saturating_duration_since(self.base).as_nanos() / self.tick.as_nanos()) as u64;
        if owed > self.done + MAX_BEHIND {
            self.reset(now);
            return 1;
        }
        owed.saturating_sub(self.done) as u32
    }

    /// Records that a tick ran.
    pub fn ran(&mut self) {
        self.done += 1;
    }

    /// Starts counting again from `now` (after a pause).
    pub fn reset(&mut self, now: Instant) {
        self.base = now;
        self.base_tick = self.done;
    }

    /// How long until the next tick is due.
    pub fn until_next(&self, now: Instant) -> Duration {
        let next = self.base + self.tick * (self.done + 1 - self.base_tick) as u32;
        next.saturating_duration_since(now)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ticks_follow_the_clock_and_a_stall_resets_it() {
        let mut c = FixedStep::new(50);
        let t0 = c.base;
        assert_eq!(c.due(t0), 0);
        assert_eq!(c.due(t0 + Duration::from_millis(45)), 2);
        c.ran();
        c.ran();
        assert_eq!(c.due(t0 + Duration::from_millis(59)), 0);
        assert_eq!(
            c.until_next(t0 + Duration::from_millis(50)),
            Duration::from_millis(10)
        );
        assert_eq!(c.due(t0 + Duration::from_millis(60)), 1);
        // a long stall: one tick, then counting from there
        assert_eq!(c.due(t0 + Duration::from_secs(5)), 1);
        c.ran();
        assert_eq!(
            c.due(t0 + Duration::from_secs(5) + Duration::from_millis(19)),
            0
        );
    }
}
