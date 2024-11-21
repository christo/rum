#!/usr/bin/env bash

# post-competition key as provided on boundvariable.org
DECRYPTION_KEY="(\b.bb)(\v.vv)06FHPVboundvarHRAkz"

echo release build
cargo build --release
echo decrypting codex
# p dumps uncompressed, decrypted um image
(echo "$DECRYPTION_KEY"; echo -n p) | target/release/rum codex.umz | ./strip_prelude.py >main.um
echo main.um stripped image
