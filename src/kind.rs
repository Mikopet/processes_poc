use serde::{Deserialize, Serialize};
use std::{env, fmt, sync::LazyLock};

static KIND: LazyLock<Kind> = LazyLock::new(|| match env::var("KIND") {
    Err(env::VarError::NotPresent) => Kind::Main,
    Ok(s) => Kind::from(s),
    _ => panic!("invalid KIND"),
});

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
            s => unimplemented!("unknown KIND: `{s}`"),
        }
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
