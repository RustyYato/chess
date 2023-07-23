use std::time::Duration;

use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Instant {
    time: Duration,
}

impl Instant {
    pub fn now() -> Self {
        let time = web_sys::window()
            .expect("not in a browser")
            .performance()
            .expect("performance object not available")
            .now();
        let time = Duration::from_millis(time.trunc() as u64)
            .checked_add(Duration::from_nanos((time.fract() * 1e6) as u64))
            .unwrap_or(Duration::MAX);

        Self { time }
    }
}

impl std::ops::Add<Duration> for Instant {
    type Output = Self;

    fn add(self, rhs: Duration) -> Self::Output {
        Self {
            time: self
                .time
                .checked_add(rhs)
                .expect("Cannot overflow while adding duration to instant"),
        }
    }
}
