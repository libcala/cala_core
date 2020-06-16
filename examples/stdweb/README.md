# Cala Core Stdweb Example
This shows how to make a web app with cala-core that works with `cargo-web`.

## Install `cargo-web`
```bash
cargo install cargo-web
```

## Debugging
You have to enable the "cala_core/stdweb" feature manually:

```bash
cargo web start
```

Then open http://[::1]:8000/

## Release
```bash
cargo web deploy
```
