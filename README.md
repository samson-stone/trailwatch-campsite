# powdergate-snowops

trailwatch-campsite is an offline Rust campground-operations workbench. It models reservation ledgers, campsite inventory, permit desks, trail segments, shuttle routes, trailhead gates, maintenance orders, wildlife sightings, rental kits, program sessions, incident logs, and supply manifests for busy park systems. The codebase is intentionally substantial in src/ so fuzzing reaches real trail-plan parsing, trip-bundle replay, digest generation, and cross-module aggregation paths rather than a toy project.

The Rust code lives in src/, with reusable domain logic kept in normal library modules instead of burying behavior inside the fuzz layer. Core subsystems include reservations, campsites, permits, trails, shuttles, trailheads, maintenance, wildlife, which gives the repository multiple meaningful places where local input can be parsed, replayed, validated, aggregated, and reported on. The intent is for the project to feel like an operator-facing offline tool or workbench, not a toy sample with padded lines or dead files.

The repository also keeps fuzz/, .clusterfuzzlite/, tests, examples, and vendored dependencies in-tree so the build can stay offline and deterministic. A normal workflow is to feed a local script, bundle, packet, digest request, or replay artifact into the library or example entry point, inspect the resulting report, and then fuzz the same surface with structured seed inputs so execution reaches deeper project logic rather than stopping at a shallow header check.

Fuzzing surface currently wired into the repository:
- libfuzzer
- address
- undefined
- trail_plan_fuzzer
- trip_bundle_fuzzer
- trail_digest_fuzzer

Operationally, the repository is meant to support repeatable local builds, meaningful harness execution, and enough project context in the README for a reviewer to understand what the code is doing and why the fuzz targets are connected to real functionality.
