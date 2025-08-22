RUSTFLAGS='-C opt-level=3' cargo build --release
sudo cp ./target/release/nscript_standalone /bin/nscript
chmod +x /bin/nscript
