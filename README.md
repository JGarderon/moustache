# Moustache 

```
cargo doc --open --all --all-features --document-private-items
```

```
cargo clean && cargo run --release -- -v "...=..." --output "..." --input "..." -r 
valgrind -s --track-origins=yes --leak-check=full target/release/moustache -v "...=..." --output "..." --input "..." -r 
```

Réduction de taille : https://github.com/johnthagen/min-sized-rust?tab=readme-ov-file

```
RUSTFLAGS="-Zlocation-detail=none" cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target x86_64-unknown-linux-gnu --release && upx --best --lzma target/release/moustache && ll -h target/release/moustache
```

__... avant : 533 Ko ; après : 151 Ko__

Avec la configuration : 

```Cargo.toml
[profile.release]
strip = true
opt-level = "s"
lto = true
codegen-units = 1
panic = "abort"
```	

### Divers 

```
clear ; clear ; DEBUG=1 ./install.sh
clear ; clear ; cargo build --release && DEBUG=1 ./tests/tests.sh "target/release/moustachet" "./tests/*.test"
clear ; clear ; RUST_BACKTRACE=1 cargo run -- -r --input ./local_tests/source4.test -v myvar="very"
```

