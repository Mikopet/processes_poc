use log::*;
use std::{
    collections::HashMap,
    env::current_exe,
    path::PathBuf,
    process::{Child, Command},
    sync::{LazyLock, Mutex},
    thread::sleep,
    time::Duration,
};

use crate::kind::Kind;

static PROC: LazyLock<Mutex<HashMap<Kind, Child>>> = LazyLock::new(|| Mutex::new(HashMap::new()));
static EXE: LazyLock<PathBuf> = LazyLock::new(|| current_exe().unwrap());

pub fn bootstrap() -> Result<(), std::io::Error> {
    spawn(Kind::Broker, false)?;
    spawn(Kind::Core, false)?;
    spawn(Kind::Content, false)?;

    Ok(())
}

pub fn monitor() {
    loop {
        match wait() {
            Err(e) => panic!("respawn error: {e}"),
            Ok(true) => break,
            _ => (),
        }
    }
}

fn wait() -> Result<bool, std::io::Error> {
    let (kind, status) = 'outer: loop {
        let mut guard = PROC.lock().unwrap();

        for (kind, child) in guard.iter_mut() {
            let pid = child.id();

            // for slower stdout
            if cfg!(debug_assertions) {
                sleep(Duration::from_millis(1000));
            } else {
                sleep(Duration::from_millis(200));
            }

            match child.try_wait() {
                Ok(Some(status)) => break 'outer (*kind, status),
                Ok(None) => debug!("~p still waiting for `{kind}#{pid}`"),
                Err(e) => error!("~p error during waiting for `{kind}#{pid}`: {e}"),
            };
        }
    };

    if status.success() {
        info!("~p `{kind}` succeeded with {status}");
        PROC.lock().unwrap().remove(&kind);
        debug!("~p no longer monitoring `{kind}`!");
    } else {
        error!("~p `{kind}` failed with {status}");
        spawn(kind, true)?;
    }

    Ok(finished())
}

fn spawn(k: Kind, r: bool) -> Result<(), std::io::Error> {
    if r {
        warn!("~p respawning `{k}`");
    } else {
        trace!("~p spawning `{k}`");
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
