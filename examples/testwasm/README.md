# Setup
The following instructions are for Fedora Linux.  You may need to modify them
for your distribution.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# Install with default options
rustup target add wasm32-unknown-unknown
sudo dnf install binaryen
cargo install https
cargo install wasm-snip
```

# Build WASM
```bash
cargo build --target wasm32-unknown-unknown --release
wasm-opt --strip-debug target/wasm32-unknown-unknown/release/testwasm.wasm -o dst/testwasm.wasm
wasm-snip --snip-rust-panicking-code --snip-rust-fmt-code dst/testwasm.wasm -o dst/testwasm.wasm
```

# Run WASM
```bash
https dst/
```
