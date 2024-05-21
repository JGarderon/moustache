#!/usr/bin/env bash

ARCHI="x86_64-unknown-linux-gnu"

RUSTFLAGS="-Zlocation-detail=none" cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target "$ARCHI" --release

upx --best --lzma "target/$ARCHI/release/moustache"

# cp "target/$ARCHI/release/moustache" ~/.local/bin/moustache
sudo cp "target/$ARCHI/release/moustache" /usr/bin/moustache
sudo chmod 771 /usr/bin/moustache
