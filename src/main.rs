mod kind;
mod logger;
mod proc;

use log::*;
use std::{process::exit, thread::sleep, time::Duration};

use kind::*;
use proc::*;

fn main() {
    logger::init().expect("logger init failure");

    if Kind::current().is_main() {
        match bootstrap() {
            Ok(_) => trace!("bootstrapped successfully"),
            Err(e) => {
                error!("bootstrap error: {e}");
                unimplemented!("retry logic missing");
            }
        }
    } else {
        debug!("apply sandbox rules: {:?}", std::env::vars());

        task();
    }

    loop {
        match wait() {
            Err(e) => {
                error!("respawn error: {e}");
                unimplemented!("retry logic missing");
            }
            Ok(true) => break,
            _ => (),
        }
    }

    info!("all processes finished");
}

// segfaults or exits with valid statuscodes (0/1)
fn task() {
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

fn segfault() {
    unsafe { std::ptr::null_mut::<i32>().write(42) };
}
