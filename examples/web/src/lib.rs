use cala_core::{
    main,
    os::web::{JsFn, JsString},
};

main!(|_sys| {
    let message = JsString::new("Hello, world!");
    let alert = unsafe { JsFn::new("alert(param_a);") };

    unsafe {
        assert!(alert.call(Some(message.as_var()), None).is_none());
    }

    cala_core::ExitStatus::Success
});
