use log::{error, info, warn};
use std::{fs::File, process::exit, thread::sleep, time::Duration};

use crate::kind::Kind;

// A test task doing various situations "randomly"
pub fn random() {
    let seed = std::process::id() % 4 + 1;
    let d = Duration::from_secs(seed as u64);
    info!("~t sleeping for {d:?}");
    sleep(d);

    let status = seed % 3;
    match status {
        0 => info!("~t finished!"),
        1 => access_file(),
        _ => segfault(),
    }
}

// Throws segfault
fn segfault() {
    unsafe { std::ptr::null_mut::<i32>().write(42) };
}

// Throws sandbox violation
fn access_file() {
    // only Core can write to log
    match File::options().append(true).open("sandbox.log") {
        Ok(_) => match Kind::current() {
            Kind::Core => info!("~t accessed log for write"),
            _ => error!("~s accessed log for write (sandbox not working)!"),
        },
        Err(e) => {
            warn!("~t no write access was allowed to log: {e}");
            exit(1);
        }
    }

    // only Content can read from log
    match std::fs::read_to_string("sandbox.log") {
        Ok(_) => match Kind::current() {
            Kind::Content => info!("~t accessed log for read"),
            _ => error!("~s accessed log for read (sandbox not working)!"),
        },
        Err(e) => {
            warn!("~t no read access was allowed to log: {e}");
            exit(1);
        }
    }
}
