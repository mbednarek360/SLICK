rm Cargo.toml
rm -r cli/core
cp -r core cli/core
ln -s Cargo_cli.toml Cargo.toml
cargo build --release
rm Cargo.toml