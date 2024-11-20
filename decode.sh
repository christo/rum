#!/usr/bin/env bash

echo release build
cargo build --release
echo running codex, decrypting
target/release/rum codex.umz <key.txt >output.raw
echo stripping prelude of ascii blurb
dd if=output.raw of=main.um bs=1 skip=198
echo main.um stripped image
