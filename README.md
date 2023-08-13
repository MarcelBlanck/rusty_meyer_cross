
## WEBASM

# WEBASM Developmnt

```
cargo install wasm-server-runner
cargo run --target wasm32-unknown-unknown
```

Make sure to [Optimize for speed](https://bevy-cheatbook.github.io/platforms/wasm/size-opt.html)


# WEBASM Release
[target.wasm32-unknown-unknown]
runner = "wasm-server-runner"
``` bash
cargo install wasm-server-runner
cargo install -f wasm-bindgen-cli
cargo install -f wasm-opt
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./wasm-out/ --target web ./target/wasm32-unknown-unknown/release/meyers_cross.wasm
wasm-opt -Os -o ./out/meyers_cross_small.wasm ./out/meyers_cross_bg.wasm
cargo run --target wasm32-unknown-unknown
```