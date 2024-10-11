use std::{fmt::Arguments, sync::LazyLock};

use colored::{Color, ColoredString, Colorize};
use log::*;

use crate::Kind;

static LOGGER: Logger = Logger;
static COLOR: LazyLock<Color> = LazyLock::new(|| Color::from(Kind::current()));

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info))
}

pub struct Logger;

impl Logger {
    fn deliver(m: &Metadata, a: &Arguments) {
        println!(
            "{}",
            Self::decorate(
                a.to_string(),
                match m.level() {
                    Level::Error => Color::Red,
                    _ => *COLOR,
                }
            )
        );
    }

    fn caller() -> String {
        let mut sym = String::from("");
        backtrace::trace(|frame| {
            backtrace::resolve_frame(frame, |symbol| {
                if let Some(name) = symbol.name() {
                    sym = name.to_string();
                }
            });

            if sym.contains("processes_poc") {
                if !sym.contains("logger") {
                    sym = match *sym.split("::").collect::<Vec<&str>>() {
                        [_, m, f, _] => format!("{m}::{f}()"),
                        [_, f, _] => format!("main::{f}()"),
                        _ => String::from("unknown"),
                    };
                    return false;
                }
            }

            true // keep going to the next frame
        });
        sym
    }

    fn task() -> &'static str {
        if Self::caller().contains("task") {
            "="
        } else {
            " "
        }
    }

    fn kind() -> String {
        format!("[{}#{}]", Kind::current().to_string(), std::process::id())
    }

    fn decorate(msg: String, c: Color) -> ColoredString {
        format!(
            "{:>16} | {:<13} |{} {msg}",
            Self::kind(),
            Self::caller(),
            Self::task(),
        )
        .color(c)
    }
}

impl Log for Logger {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn log(&self, r: &Record) {
        Self::deliver(r.metadata(), r.args());
    }

    fn flush(&self) {}
}
