# Cala Core Web Example
This shows how to make a backend-agnostic web app with cala-core.

## Cala Backend
```bash
# cargo install cargo-cala
cargo cala web
```

Then open the link it prints out.

## Stdweb Backend
You have to enable the "cala_core/stdweb" feature manually:

```bash
# cargo install cargo-web
cargo web start --features=cala_core/stdweb
```

Then open http://[::1]:8000/

## Wasm-Bindgen Backend
You have to enable the "cala_core/wasm-bindgen" feature manually:

```bash
# curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
# cargo install https
wasm-pack build --target web -d ./wasm-pack/pkg -- --features=cala_core/wasm-bindgen
http ./wasm-pack/
```

Then open http://0.0.0.0:8000/
