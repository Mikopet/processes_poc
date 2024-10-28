mod kind;
mod logger;
mod proc;
mod sandbox;
mod task;

use crate::kind::Kind;

fn main() {
    logger::init().expect("logger init failure");

    match sandbox::restrict() {
        Err(e) => panic!("sandbox error: {e}"),
        _ => match Kind::current() {
            Kind::Main => match proc::bootstrap() {
                Err(e) => panic!("bootstrap error: {e}"),
                _ => proc::monitor(),
            },
            _ => task::run(),
        },
    }

    log::trace!("~p graceful termination");
}
