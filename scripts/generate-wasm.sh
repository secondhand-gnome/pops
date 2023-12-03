# see https://bevy-cheatbook.github.io/platforms/wasm/webpage.html
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --no-typescript --target web --out-dir ./out/ --out-name "pops" ./target/wasm32-unknown-unknown/release/pops.wasm

# Copy the assets over
rm -rf out/assets
cp -r assets out/assets

# Generate a ZIP of the web files
cd out
tar -a -c -f pops.zip *.html *.wasm *.js assets
cd -