use trailwatch_campsite::{TrailwatchDesk, TrailEngine};

#[test]
fn run_and_report_paths_work() {
    let text = r#"
MODULE reservations
FAMILY tent
SHARD 17
RECORDS 88
SEED 5
LIMIT 3
END
"#;

    let digests = TrailEngine::inspect_trails(text).expect("script should parse");
    assert_eq!(digests.len(), 1);
    assert!(TrailwatchDesk::digest_trails(text).expect("digest").contains("trailwatch total="));
}