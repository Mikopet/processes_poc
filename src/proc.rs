use lazy_static::lazy_static;
use log::*;
use std::{
    collections::HashMap,
    env::current_exe,
    path::PathBuf,
    process::{Child, Command},
    sync::Mutex,
    thread::sleep,
    time::Duration,
};

use crate::{error, Kind};

lazy_static! {
    static ref PROC: Mutex<HashMap<Kind, Child>> = Mutex::new(HashMap::new());
    static ref EXE: PathBuf = current_exe().unwrap();
}

pub fn bootstrap() -> Result<(), std::io::Error> {
    spawn(Kind::Broker, false)?;
    spawn(Kind::Core, false)?;
    spawn(Kind::Content, false)?;

    Ok(())
}

pub fn wait() -> Result<bool, std::io::Error> {
    let (kind, status) = 'outer: loop {
        let mut guard = PROC.lock().unwrap();

        for (kind, child) in guard.iter_mut() {
            sleep(Duration::from_secs(1)); // for slow stdout

            match child.try_wait() {
                Ok(Some(status)) => break 'outer (*kind, status),
                Ok(None) => debug!("`{kind}` is running with pid #{}", child.id()),
                Err(e) => error!("error attempting to wait for `{kind}`: {e}"),
            };
        }
    };

    if status.success() {
        info!("`{kind}` exited successfully {status}");
        PROC.lock().unwrap().remove(&kind);
        debug!("no longer monitoring `{kind}`!");
    } else {
        error!("`{kind}` exited with failure {status}");
        spawn(kind, true)?;
    }

    Ok(finished())
}

fn spawn(k: Kind, r: bool) -> Result<(), std::io::Error> {
    if r {
        warn!("respawning `{k}`");
    } else {
        trace!("spawning `{k}`");
    }

    let child = Command::new(&*EXE)
        .env_clear()
        .env("KIND", k.to_string())
        // .env sandboxing
        .spawn()?;

    PROC.lock().unwrap().insert(k, child);

    Ok(())
}

fn finished() -> bool {
    PROC.lock().unwrap().is_empty()
}
