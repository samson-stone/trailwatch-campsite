#![no_main]
use libfuzzer_sys::fuzz_target;
use trailwatch_campsite::TrailwatchDesk;

fuzz_target!(|data: &[u8]| {
    if let Ok(text) = std::str::from_utf8(data) {
        let _ = TrailwatchDesk::digest_trails(text);
        let _ = TrailwatchDesk::compact_trails();
    }
});