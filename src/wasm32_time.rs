use universal_time::{Instant, MonotonicClock, SystemTime, WallClock, define_time_provider};

struct WebTimeProvider;

impl WallClock for WebTimeProvider {
    fn system_time(&self) -> SystemTime {
        let now = web_time::SystemTime::now();
        SystemTime::from_unix_duration(now.elapsed().unwrap())
    }
}

impl MonotonicClock for WebTimeProvider {
    fn instant(&self) -> Instant {
        let now = web_time::Instant::now();
        Instant::from_ticks(now.elapsed())
    }
}

define_time_provider!(WebTimeProvider);
