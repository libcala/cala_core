#![deny(unsafe_code)]

use pasts::prelude::*;

#[allow(unsafe_code)]
mod timer;
#[macro_use]
#[allow(unsafe_code)]
mod alert;

async fn task_a() {
    let mut future = timer::JsTimer::new(42, 750);
    loop {
        let result = (&mut future).await;
        alert!("Waited 3/4 a second to get: {}", result);
    }
}

async fn task_b() {
    let mut future = timer::JsTimer::new(42, 1500);
    loop {
        let result = (&mut future).await;
        alert!("Waited 1.5 a second to get: {}", result);
    }
}

async fn webapp() {
    alert!("Hello, world!");
    [task_a().fut(), task_b().fut()].select().await;
}

fn main() {
    // Set panic handler for clean prints.
    cala_core::os::web::panic_hook();
    // Start the executor
    cala_core::os::web::block_on(webapp());
}
