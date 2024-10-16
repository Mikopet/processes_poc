use colored::Color;
use std::{
    env::VarError,
    fmt::{Display, Formatter, Result},
    sync::LazyLock,
};

static KIND: LazyLock<Kind> = LazyLock::new(|| match std::env::var("KIND") {
    Err(VarError::NotPresent) => Kind::Main,
    Ok(k) => Kind::from(k),
    _ => panic!("invalid KIND"),
});

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Kind {
    Main,
    Broker,
    Core,
    Content,
}

impl Kind {
    pub fn current() -> &'static Self {
        &KIND
    }
}

impl From<String> for Kind {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Broker" => Self::Broker,
            "Core" => Self::Core,
            "Content" => Self::Content,
            _ => unimplemented!("process type is unknown"),
        }
    }
}

impl Display for Kind {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self)
    }
}

impl From<&Kind> for Color {
    fn from(k: &Kind) -> Self {
        match k {
            Kind::Main => Color::Magenta,
            Kind::Broker => Color::Green,
            Kind::Core => Color::Cyan,
            Kind::Content => Color::Yellow,
        }
    }
}
