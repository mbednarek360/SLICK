rm Cargo.toml
rm -r web/core
cp -r core web/core
ln -s Cargo_web.toml Cargo.toml
rm -r web/pkg
wasm-pack build --target web --release
mv pkg web
rm Cargo.toml