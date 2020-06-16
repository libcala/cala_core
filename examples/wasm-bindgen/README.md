# Cala Core Web Example
This shows how to make a backend-agnostic web app with cala-core.

## Install `wasm-pack` and `https`
```bash
# curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
# cargo install https
```

## Building The Package
```bash
wasm-pack build --target web -d ./wasm-pack/pkg
http ./wasm-pack/
```

Then open http://0.0.0.0:8000/
