#![no_main]
use libfuzzer_sys::fuzz_target;
use trailwatch_campsite::TrailEngine;

fuzz_target!(|data: &[u8]| {
    if let Ok(text) = std::str::from_utf8(data) {
        let _ = TrailEngine::inspect_trip_bundles(text);
    }
});