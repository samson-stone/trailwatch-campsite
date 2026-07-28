use crate::script::{TrailAction, TrailPlan, ScriptError};
use crate::{
    rentals, incidents, supplies, campsites, maintenance, trails, wildlife, permits, reservations,
    programs, shuttles, trailheads,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrailDigest {
    pub module: String,
    pub family_count: usize,
    pub weighted_total: u64,
    pub nearest_key: Option<String>,
    pub notes: Vec<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct TrailEngine;

impl TrailEngine {
    pub fn inspect_trails(text: &str) -> Result<Vec<TrailDigest>, ScriptError> {
        let run = TrailPlan::parse(text)?;
        Ok(run.actions.iter().map(Self::digest_action).collect())
    }

    pub fn inspect_trip_bundles(text: &str) -> Result<Vec<Vec<TrailDigest>>, ScriptError> {
        let mut bundles = Vec::new();
        for block in text.split("\n===\n") {
            if block.trim().is_empty() {
                continue;
            }
            bundles.push(Self::inspect_trails(block)?);
        }
        Ok(bundles)
    }

    fn digest_action(action: &TrailAction) -> TrailDigest {
        match action.module.as_str() {
            "reservations" => digest_generic(action, reservations::reservations_family_window(&action.family, action.limit).len(), reservations::reservations_weighted_total(action.seed), reservations::reservations_nearest(action.shard, action.survey).map(|item| item.key.to_string()), reservations::reservations_note_digest(action.limit.min(8))),
            "campsites" => digest_generic(action, campsites::campsites_family_window(&action.family, action.limit).len(), campsites::campsites_weighted_total(action.seed), campsites::campsites_nearest(action.shard, action.survey).map(|item| item.key.to_string()), campsites::campsites_note_digest(action.limit.min(8))),
            "permits" => digest_generic(action, permits::permits_family_window(&action.family, action.limit).len(), permits::permits_weighted_total(action.seed), permits::permits_nearest(action.shard, action.survey).map(|item| item.key.to_string()), permits::permits_note_digest(action.limit.min(8))),
            "trails" => digest_generic(action, trails::trails_family_window(&action.family, action.limit).len(), trails::trails_weighted_total(action.seed), trails::trails_nearest(action.shard, action.survey).map(|item| item.key.to_string()), trails::trails_note_digest(action.limit.min(8))),
            "shuttles" => digest_generic(action, shuttles::shuttles_family_window(&action.family, action.limit).len(), shuttles::shuttles_weighted_total(action.seed), shuttles::shuttles_nearest(action.shard, action.survey).map(|item| item.key.to_string()), shuttles::shuttles_note_digest(action.limit.min(8))),
            "trailheads" => digest_generic(action, trailheads::trailheads_family_window(&action.family, action.limit).len(), trailheads::trailheads_weighted_total(action.seed), trailheads::trailheads_nearest(action.shard, action.survey).map(|item| item.key.to_string()), trailheads::trailheads_note_digest(action.limit.min(8))),
            "maintenance" => digest_generic(action, maintenance::maintenance_family_window(&action.family, action.limit).len(), maintenance::maintenance_weighted_total(action.seed), maintenance::maintenance_nearest(action.shard, action.survey).map(|item| item.key.to_string()), maintenance::maintenance_note_digest(action.limit.min(8))),
            "wildlife" => digest_generic(action, wildlife::wildlife_family_window(&action.family, action.limit).len(), wildlife::wildlife_weighted_total(action.seed), wildlife::wildlife_nearest(action.shard, action.survey).map(|item| item.key.to_string()), wildlife::wildlife_note_digest(action.limit.min(8))),
            "rentals" => digest_generic(action, rentals::rentals_family_window(&action.family, action.limit).len(), rentals::rentals_weighted_total(action.seed), rentals::rentals_nearest(action.shard, action.survey).map(|item| item.key.to_string()), rentals::rentals_note_digest(action.limit.min(8))),
            "programs" => digest_generic(action, programs::programs_family_window(&action.family, action.limit).len(), programs::programs_weighted_total(action.seed), programs::programs_nearest(action.shard, action.survey).map(|item| item.key.to_string()), programs::programs_note_digest(action.limit.min(8))),
            "incidents" => digest_generic(action, incidents::incidents_family_window(&action.family, action.limit).len(), incidents::incidents_weighted_total(action.seed), incidents::incidents_nearest(action.shard, action.survey).map(|item| item.key.to_string()), incidents::incidents_note_digest(action.limit.min(8))),
            "supplies" => digest_generic(action, supplies::supplies_family_window(&action.family, action.limit).len(), supplies::supplies_weighted_total(action.seed), supplies::supplies_nearest(action.shard, action.survey).map(|item| item.key.to_string()), supplies::supplies_note_digest(action.limit.min(8))),
            _ => TrailDigest { module: action.module.clone(), family_count: 0, weighted_total: 0, nearest_key: None, notes: Vec::new() },
        }
    }
}

fn digest_generic(action: &TrailAction, family_count: usize, weighted_total: u64, nearest_key: Option<String>, notes: Vec<String>) -> TrailDigest {
    TrailDigest {
        module: action.module.clone(),
        family_count,
        weighted_total,
        nearest_key,
        notes,
    }
}