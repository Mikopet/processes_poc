mod kind;
mod logger;
mod proc;
mod task;

use crate::kind::Kind;
use log::*;

fn main() {
    logger::init().expect("logger init failure");

    match Kind::current() {
        Kind::Main => match proc::bootstrap() {
            Err(e) => panic!("bootstrap error: {e}"),
            _ => proc::monitor(),
        },
        _ => task::random(),
    }

    info!("all processes finished");
}
