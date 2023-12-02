# see https://bevy-cheatbook.github.io/platforms/wasm/webpage.html
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --no-typescript --target web --out-dir ./out/ --out-name "pops" ./target/wasm32-unknown-unknown/release/pops.wasm

# Generate a ZIP of the web files
cd out
tar -a -c -f pops.zip *.html *.wasm *.js
cd -