#!/usr/bin/env bash

echo release build
cargo build --release
echo running codex, decrypting
cat key.txt $(echo p) | target/release/rum codex.umz >output.raw
echo stripping prelude of 195 bytes of ascii blurb
dd if=output.raw of=main.um bs=1 skip=195
echo main.um stripped image
