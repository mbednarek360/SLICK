rm Cargo.toml
ln -s Cargo_web.toml Cargo.toml
rm -r web/pkg
wasm-pack build --target web --release
mv pkg web