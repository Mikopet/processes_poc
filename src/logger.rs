use colored::{Color, ColoredString, Colorize};
use log::*;

static LOGGER: Logger = Logger;

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info))
}

pub struct Logger;

impl Logger {
    fn deliver(m: &Metadata, a: &std::fmt::Arguments) {
        println!("{}", Self::decorate(a.to_string(), m.level(),));
    }

    fn decorate(msg: String, l: Level) -> String {
        let process = format!(
            "{:>7}#{:<6}",
            crate::Kind::current().to_string(),
            std::process::id()
        );
        let prefix = format!(
            "{} {} ",
            process.bright_black(),
            Self::delimiter(l, msg.contains('\n'))
        );

        let msg = msg.replace('\n', format!("\n{prefix}").as_str());

        format!("{prefix}{}", msg.color(Self::color(l)))
    }

    fn color(l: Level) -> Color {
        match l {
            Level::Error => Color::BrightRed,
            Level::Warn => Color::BrightYellow,
            Level::Info => Color::BrightWhite,
            Level::Debug => Color::BrightCyan,
            Level::Trace => Color::BrightMagenta,
        }
    }

    fn delimiter(l: Level, m: bool) -> ColoredString {
        match l {
            Level::Error => "!".red().bold(),
            _ => match m {
                false => "|".white().bold(),
                _ => "»".white(),
            },
        }
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
