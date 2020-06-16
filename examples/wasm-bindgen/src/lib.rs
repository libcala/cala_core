use wasm_bindgen::prelude::*;
use cala_core::os::web::{JsFn, JsString};

#[wasm_bindgen]
pub fn wasm_main() {
    let message = JsString::new("Hello, world!");
    let alert = unsafe { JsFn::new("alert(param_a);") };

    unsafe {
        assert!(alert.call(Some(message.as_var()), None).is_none());
    }
}
