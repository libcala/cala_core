#[macro_use]
extern crate cala_core;

// Start the program.
start!();

// Uncomment for wasm-bindgen support.
/*mod hook {
    use wasm_bindgen::prelude::*;
    #[wasm_bindgen]
    pub fn start() {
        cala_core::_macro::start(Box::pin(super::start()))
    }
}*/

// Called by the start!() macro.
async fn start() {
    log!("Hello, world! ☺");
}
