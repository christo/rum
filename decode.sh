#!/usr/bin/env bash

echo release build
cargo build --release
echo running codex, decrypting
target/release/rum codex.umz <key.txt >output.raw
echo stripping prelude of ascii blurb
./strip_prelude.py < output.raw >main.um
echo main.um stripped image
