use cala_core::os::web::{JsFn, JsString};

macro_rules! alert {
    ($a:literal) => {
        $crate::alert::alert(format_args!($a))
    };
    ($a:literal, $b:tt) => {
        $crate::alert::alert(format_args!($a, $b))
    };
}

pub(crate) fn alert(args: std::fmt::Arguments) {
    let alert = unsafe { JsFn::new("console.log(param_a);") };

    let message = JsString::new(&format!("{}", args));

    unsafe {
        assert!(alert.call(Some(message.as_var()), None).is_none());
    }
}
