# Moustache 

```
cargo doc --open --all --all-features --document-private-items
```

```
cargo clean && cargo run --release -- -v "...=..." --output "..." --input "..." -r 
valgrind -s --track-origins=yes --leak-check=full target/release/moustache -v "...=..." --output "..." --input "..." -r 
```

