#![deny(unsafe_code)]

#[allow(unsafe_code)]
mod timer;
#[macro_use]
#[allow(unsafe_code)]
mod alert;

async fn webapp() {
    alert!("Hello, world!");
    let mut future = timer::JsTimer::new(42, 750);
    loop {
        let result = (&mut future).await;
        alert!("Waited 3/4 a second to get: {}", result);
    }
}

fn main() {
    // Set panic handler for clean prints.
    cala_core::os::web::panic_hook();
    // Start the executor
    cala_core::os::web::block_on(webapp());
}
