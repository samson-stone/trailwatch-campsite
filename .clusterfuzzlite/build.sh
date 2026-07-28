#!/bin/bash -eu

cd "$SRC"

cargo fuzz build trail_plan_fuzzer --release
cargo fuzz build trip_bundle_fuzzer --release
cargo fuzz build trail_digest_fuzzer --release

cp fuzz/target/x86_64-unknown-linux-gnu/release/trail_plan_fuzzer "$OUT/trail_plan_fuzzer"
cp fuzz/target/x86_64-unknown-linux-gnu/release/trip_bundle_fuzzer "$OUT/trip_bundle_fuzzer"
cp fuzz/target/x86_64-unknown-linux-gnu/release/trail_digest_fuzzer "$OUT/trail_digest_fuzzer"

python3 - <<'PY'
import os
import zipfile

out = os.environ["OUT"]
targets = ["trail_plan_fuzzer", "trip_bundle_fuzzer", "trail_digest_fuzzer"]
for name in targets:
    corpus_dir = os.path.join("fuzz", "corpus", name)
    zip_path = os.path.join(out, f"{name}_seed_corpus.zip")
    if os.path.isdir(corpus_dir):
        with zipfile.ZipFile(zip_path, "w", zipfile.ZIP_DEFLATED) as zf:
            for seed_name in sorted(os.listdir(corpus_dir)):
                path = os.path.join(corpus_dir, seed_name)
                if os.path.isfile(path):
                    zf.write(path, seed_name)
PY