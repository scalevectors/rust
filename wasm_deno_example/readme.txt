Example followed from
https://dev.to/lampewebdev/writing-webassembly-in-rust-and-runing-it-in-deno-144j?WT.mc_id=link-twitter-jeliknes


build:
cargo build --target wasm32-unknown-unknown
wasm-gc target/wasm32-unknown-unknown/debug/wasm_deno_example.wasm
deno run --allow-read main.ts
