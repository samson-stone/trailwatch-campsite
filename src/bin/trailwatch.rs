use trailwatch_campsite::TrailwatchDesk;

fn main() {
    let report = TrailwatchDesk::report();
    println!("{}", TrailwatchDesk::compact_trails());
    println!("purposes={}", report.purpose_lines.len());
    println!("summaries={}", report.summaries.len());
}