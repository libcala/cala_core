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
wasm-opt --strip-debug target/wasm32-unknown-unknown/release/testwasm_lib.wasm -o dst/testwasm_lib.wasm
wasm-snip --snip-rust-panicking-code --snip-rust-fmt-code dst/testwasm_lib.wasm -o dst/testwasm_lib.wasm
```

# Run WASM
```bash
https dst/
```

# With Wasm-Pack And Wasm-Bindgen
First, in Cargo.toml, uncomment the wasm-bindgen dependency, and feature on
cala\_core.

Second, uncomment the `hook` module in testwasm.rs while also commenting out the
`start!()` macro.

And, finally, run this command:

```bash
wasm-pack build --target web -d ./wasm-pack/pkg && http ./wasm-pack/
```
