use log::{info, warn};
use std::{process::exit, thread::sleep, time::Duration};

// A test task doing various situations "randomly"
pub fn random() {
    let seed = std::process::id() % 4 + 1;
    let d = Duration::from_secs(seed as u64);
    info!("sleeping for {d:?}");
    sleep(d);

    let status = seed % 3;
    match status {
        0 => info!("finished!"),
        1 => warn!("soft fail!"),
        _ => segfault(),
    }
    exit(status as i32);
}

// Throws segfault
fn segfault() {
    unsafe { std::ptr::null_mut::<i32>().write(42) };
}
