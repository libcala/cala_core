use cala_core::os::web::{JsFn, JsString};

fn main() {
    let message = JsString::new("Hello, world!");
    let alert = unsafe { JsFn::new("alert(param_a);") };

    unsafe {
        assert!(alert.call(Some(message.as_var()), None).is_none());
    }
}
