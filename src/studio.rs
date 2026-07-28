use crate::kernel::TrailEngine;
use crate::script::ScriptError;
use crate::{
    rentals, incidents, supplies, campsites, maintenance, trails, wildlife, permits, reservations,
    programs, shuttles, trailheads,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrailDigestLane {
    pub lane: String,
    pub weight: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrailCard {
    pub purpose_lines: Vec<String>,
    pub summaries: Vec<String>,
    pub digests: Vec<TrailDigestLane>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct TrailwatchDesk;

impl TrailwatchDesk {
    pub fn report() -> TrailCard {
        let purpose_lines = vec![
            reservations::reservations_purpose().to_string(),
            campsites::campsites_purpose().to_string(),
            permits::permits_purpose().to_string(),
            trails::trails_purpose().to_string(),
            shuttles::shuttles_purpose().to_string(),
            trailheads::trailheads_purpose().to_string(),
            maintenance::maintenance_purpose().to_string(),
            wildlife::wildlife_purpose().to_string(),
            rentals::rentals_purpose().to_string(),
            programs::programs_purpose().to_string(),
            incidents::incidents_purpose().to_string(),
            supplies::supplies_purpose().to_string(),
        ];
        let summaries = vec![
            reservations::reservations_summary(),
            campsites::campsites_summary(),
            permits::permits_summary(),
            trails::trails_summary(),
            shuttles::shuttles_summary(),
            trailheads::trailheads_summary(),
            maintenance::maintenance_summary(),
            wildlife::wildlife_summary(),
            rentals::rentals_summary(),
            programs::programs_summary(),
            incidents::incidents_summary(),
            supplies::supplies_summary(),
        ];
        let digests = vec![
            TrailDigestLane { lane: "reservations".into(), weight: reservations::reservations_weighted_total(3) },
            TrailDigestLane { lane: "campsites".into(), weight: campsites::campsites_weighted_total(5) },
            TrailDigestLane { lane: "permits".into(), weight: permits::permits_weighted_total(7) },
            TrailDigestLane { lane: "trails".into(), weight: trails::trails_weighted_total(11) },
            TrailDigestLane { lane: "shuttles".into(), weight: shuttles::shuttles_weighted_total(13) },
            TrailDigestLane { lane: "trailheads".into(), weight: trailheads::trailheads_weighted_total(17) },
            TrailDigestLane { lane: "maintenance".into(), weight: maintenance::maintenance_weighted_total(19) },
            TrailDigestLane { lane: "wildlife".into(), weight: wildlife::wildlife_weighted_total(23) },
            TrailDigestLane { lane: "rentals".into(), weight: rentals::rentals_weighted_total(29) },
            TrailDigestLane { lane: "programs".into(), weight: programs::programs_weighted_total(31) },
            TrailDigestLane { lane: "incidents".into(), weight: incidents::incidents_weighted_total(37) },
            TrailDigestLane { lane: "supplies".into(), weight: supplies::supplies_weighted_total(41) },
        ];
        TrailCard { purpose_lines, summaries, digests }
    }

    pub fn compact_trails() -> String {
        let report = Self::report();
        let total: u64 = report.digests.iter().map(|item| item.weight).sum();
        let lanes = report
            .digests
            .iter()
            .map(|item| format!("{}={}", item.lane, item.weight))
            .collect::<Vec<_>>()
            .join(" | ");
        format!("trailwatch total={} {}", total, lanes)
    }

    pub fn digest_trails(text: &str) -> Result<String, ScriptError> {
        let report = TrailEngine::inspect_trails(text)?;
        let total: u64 = report.iter().map(|item| item.weighted_total).sum();
        let lanes = report
            .iter()
            .map(|item| format!("{}:{}:{}", item.module, item.family_count, item.weighted_total))
            .collect::<Vec<_>>()
            .join(";");
        Ok(format!("trailwatch total={} {}", total, lanes))
    }
}