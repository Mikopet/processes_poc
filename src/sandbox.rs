use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::sync::LazyLock;

use landlock::{path_beneath_rules, Access, AccessFs, AccessNet, NetPort, ABI};
use landlock::{RestrictionStatus, RulesetError};
use landlock::{Ruleset, RulesetAttr, RulesetCreated, RulesetCreatedAttr, RulesetStatus};
use serde::{Deserialize, Serialize};

use log::{debug, error, info, warn};

use crate::kind::Kind;

const ABI: ABI = ABI::V5;

pub static SBOX_RULES: LazyLock<HashMap<Kind, Sandbox>> =
    LazyLock::new(|| match std::fs::read_to_string("sandbox.toml") {
        Ok(s) => match toml::from_str(&s) {
            Ok(t) => t,
            Err(e) => panic!("sandbox error: {e}"),
        },
        Err(e) => panic!("sandbox error: {e}"),
    });

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Sandbox {
    fs_ro: Vec<String>,
    fs_rw: Vec<String>,
    tcp_bind: Vec<u16>,
    tcp_conn: Vec<u16>,
}

impl Sandbox {
    fn default_ruleset(&self) -> Result<Ruleset, RulesetError> {
        let mut rs = Ruleset::default();
        let rs_ref = rs.borrow_mut();

        // Restrict FS accesses
        rs_ref.handle_access(AccessFs::from_all(ABI))?;

        // Restrict NET accesses
        if !self.tcp_bind.is_empty() {
            rs_ref.handle_access(AccessNet::BindTcp)?;
        }
        if !self.tcp_conn.is_empty() {
            rs_ref.handle_access(AccessNet::ConnectTcp)?;
        }

        Ok(rs)
    }

    fn create_ruleset(&self) -> Result<RulesetCreated, RulesetError> {
        let rs = self.default_ruleset()?.create()?;

        rs.add_rules(path_beneath_rules(&self.fs_ro, AccessFs::from_read(ABI)))?
            .add_rules(path_beneath_rules(&self.fs_rw, AccessFs::from_all(ABI)))?
            .add_rules(
                self.tcp_bind
                    .iter()
                    .map(|p| Ok(NetPort::new(*p, AccessNet::BindTcp)))
                    .collect::<Vec<Result<NetPort, RulesetError>>>()
                    .into_iter(),
            )?
            .add_rules(
                self.tcp_conn
                    .iter()
                    .map(|p| Ok(NetPort::new(*p, AccessNet::ConnectTcp)))
                    .collect::<Vec<Result<NetPort, RulesetError>>>()
                    .into_iter(),
            )
    }
}

pub fn restrict() -> Result<RestrictionStatus, RulesetError> {
    let sb = Sandbox::from(Kind::current());

    let status = sb.create_ruleset()?.restrict_self()?;

    debug!("~s Sandbox rules applied: {:#?}", status);

    match status.ruleset {
        RulesetStatus::FullyEnforced => info!("~s Fully sandboxed."),
        RulesetStatus::PartiallyEnforced => warn!("~s Partially sandboxed."),
        RulesetStatus::NotEnforced => error!("~s Not sandboxed! Please update your kernel."),
    }

    Ok(status)
}

impl From<&Kind> for Sandbox {
    fn from(k: &Kind) -> Self {
        match SBOX_RULES.get(k) {
            Some(sb) => Self {
                fs_ro: sb.fs_ro.clone(),
                fs_rw: sb.fs_rw.clone(),
                tcp_bind: sb.tcp_bind.clone(),
                tcp_conn: sb.tcp_conn.clone(),
            },
            None => Sandbox::default(),
        }
    }
}
