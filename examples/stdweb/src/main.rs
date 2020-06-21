#![deny(unsafe_code)]

use cala_core::os::web::JsExec;

#[allow(unsafe_code)]
mod timer;
#[macro_use]
#[allow(unsafe_code)]
mod alert;

use timer::JsTimer;

async fn webapp() {
    alert!("Hello, world!");
    let result = JsTimer::new(42, 750).await;
    alert!("Waited 3/4 a second to get: {}", result);
}

thread_local!(
    static EXECUTOR: JsExec = JsExec::new(webapp());
);

// Executor callback.
#[no_mangle]
extern "C" fn wasm_exec() {
    // Initial 
    EXECUTOR.with(|executor| executor.wake());
}

fn main() {
    // Set panic handler for clean prints.
    cala_core::os::web::panic_hook();
    // Start the executor
    wasm_exec();
}
