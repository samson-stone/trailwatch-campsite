# trailwatch-campsite

trailwatch-campsite is an offline Rust campground-operations workbench. It
models reservation ledgers, campsite inventory, permit desks, trail segments,
shuttle routes, trailhead gates, maintenance orders, wildlife sightings,
rental kits, program sessions, incident logs, and supply manifests for busy
park systems.

The codebase is intentionally substantial in src/ so fuzzing reaches real
trail-plan parsing, trip-bundle replay, digest generation, and cross-module
aggregation paths rather than a toy project.