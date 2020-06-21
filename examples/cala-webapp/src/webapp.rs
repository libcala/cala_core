#![deny(unsafe_code)]

cala_core::exec!(webapp);

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
