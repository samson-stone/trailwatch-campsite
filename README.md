# powdergate-snowops

trailwatch-campsite is an offline Rust campground-operations workbench. It models reservation ledgers, campsite inventory, permit desks, trail segments, shuttle routes, trailhead gates, maintenance orders, wildlife sightings, rental kits, program sessions, incident logs, and supply manifests for busy park systems.

The Rust code lives in src/, where library modules handle the domain model, input parsing, state updates, validation, and reporting paths that operators would use during ordinary local review. Core subsystems include reservations, campsites, permits, trails, shuttles, trailheads, maintenance, wildlife, which lets the repository accept structured local inputs, replay workflow state, validate records, aggregate summaries, and render reports or planning output across several entry points.

The repository also keeps fuzz/, .clusterfuzzlite/, tests, examples, and vendored dependencies in-tree so the project can build and run from a clean offline checkout. A normal workflow is to feed a local script, bundle, packet, digest request, or replay artifact into the library or example entry point, inspect the resulting report, and then fuzz the same surface with structured seed inputs so those same code paths are exercised under mutation as well.

Fuzzing surface currently wired into the repository:
- trail_plan_fuzzer
- trip_bundle_fuzzer
- trail_digest_fuzzer

Operationally, the repository is meant to support repeatable local builds, meaningful harness execution, and enough project context in the README for a reviewer to understand what the code is doing and how the fuzz targets connect to ordinary project behavior.
