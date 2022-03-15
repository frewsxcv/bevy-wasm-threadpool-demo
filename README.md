```
cargo build --target wasm32-unknown-unknown

wasm-bindgen --out-name wasm_example --out-dir out --target web target/wasm32-unknown-unknown/debug/bevy_thread_test.wasm

basic-http-server
```
