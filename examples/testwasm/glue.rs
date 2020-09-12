//! Cross-platform glue

include!("src/testwasm.rs");

// Uncomment for wasm-bindgen support.
/*mod hook {
    use wasm_bindgen::prelude::*;
    #[wasm_bindgen]
    pub fn start() {
        cala_core::_macro::start(Box::pin(super::start()))
    }
}*/
