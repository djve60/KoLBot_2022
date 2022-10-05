// Copyright D. Evans (djve60+kolbot@gmail.com).

// A collection of signal hanling functions for kolbot_2022.
// Well, the only call but it makes main.rs easier to read.

// If you don't know what these are be very careful.
// See http://www.apuebook.com/ for the best guide.
// For a quick introduction to issues see https://docs.rs/signal-hook.
// Examples: https://rust-cli.github.io/book/in-depth/signals.html

use signal_hook::consts::{SIGINT, SIGQUIT, SIGTERM};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

pub fn create_signal_watcher() -> Arc<AtomicBool> {
    let shutdown_flag = Arc::new(AtomicBool::new(false));
    // Signal #2, and CTRL-C (if you're lucky).
    signal_hook::flag::register(SIGINT, Arc::clone(&shutdown_flag))
        .expect("Couln't register SIGINT handler.");
    // Signal #3.
    signal_hook::flag::register(SIGQUIT, Arc::clone(&shutdown_flag))
        .expect("Couln't register SIGQUIT handler.");
    // Signal #15.
    signal_hook::flag::register(SIGTERM, Arc::clone(&shutdown_flag))
        .expect("Couln't register SIGTERM handler.");

    shutdown_flag
}
