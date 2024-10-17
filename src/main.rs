mod kind;
mod logger;
mod proc;
mod task;

use crate::kind::Kind;

fn main() {
    logger::init().expect("logger init failure");

    match Kind::current() {
        Kind::Main => match proc::bootstrap() {
            Err(e) => panic!("bootstrap error: {e}"),
            _ => proc::monitor(),
        },
        _ => task::random(),
    }

    log::trace!("~p graceful termination");
}
