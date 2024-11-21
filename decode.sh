#!/usr/bin/env bash

echo release build
cargo build --release
echo decrypting codex
(cat key.txt; echo p) | target/release/rum codex.umz | ./strip_prelude.py >main.um
echo main.um stripped image
