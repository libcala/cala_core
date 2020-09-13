#[macro_use]
extern crate cala_core;

// Start the program.
start!();

// Module for testing.
mod module;

// Called by the start!() macro.
async fn start() {
    log!("Hello, world! ☺");
}
