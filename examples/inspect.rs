use trailwatch_campsite::{TrailwatchDesk, TrailEngine};

fn main() {
    let text = r#"
MODULE campsites
FAMILY hikein
SHARD 9
RECORDS 144
SEED 7
LIMIT 4
END

MODULE incidents
FAMILY weather
SHARD 12
RECORDS 188
SEED 11
LIMIT 5
END
"#;

    let digests = TrailEngine::inspect_trails(text).unwrap();
    println!("action-count={}", digests.len());
    println!("{}", TrailwatchDesk::digest_trails(text).unwrap());
}