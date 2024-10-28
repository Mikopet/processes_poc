use log::{debug, error, info, trace, warn};
use std::{fs::File, process::exit, thread::sleep, time::Duration};

use crate::kind::Kind;

// Running the task
pub fn run() {
    let seed = std::process::id() % 4 + 1;
    let d = Duration::from_secs(seed as u64);

    loop {
        debug!("~t ... {}ing for {d:?}", Kind::current());
        sleep(d);
        if seed == 1 {
            segfault();
        }

        match Kind::current() {
            Kind::Broker => broker(),
            Kind::Core => core(),
            Kind::Content => content(),
            _ => segfault(),
        }
    }
}

// Illegal operation results in SEGFAULT
fn segfault() {
    unsafe { std::ptr::null_mut::<i32>().write(42) };
}

// Broker has no FS access
fn broker() {
    match std::fs::read_to_string("sandbox.log") {
        Ok(_) => error!("~s sandbox violation!"),
        Err(e) => warn!("~s sandbox deny! {e}"),
    }

    trace!("~t brokering done ...");
}

// Core tries to write file
fn core() {
    match File::options().append(true).open("sandbox.log") {
        Ok(_) => info!("~t file WRITE allowed!"),
        Err(e) => {
            error!("~s file WRITE disallowed! {e}");
            exit(1);
        }
    }

    trace!("~ core stuff done ...");
}

// Content tries to read file
fn content() {
    match std::fs::read_to_string("sandbox.log") {
        Ok(_) => info!("~t file READ allowed!"),
        Err(e) => {
            error!("~s file READ disallowed! {e}");
            exit(1);
        }
    }

    trace!("~t content stuff done ...");
}
