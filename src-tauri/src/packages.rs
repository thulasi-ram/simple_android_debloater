use crate::adb_cmd::{ADBCommand, ADBShell};
use anyhow::{anyhow, Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PackageState {
    Enabled,
    Uninstalled,
    Disabled,
}

impl Display for PackageState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PackageState::Enabled => write!(f, "enabled"),
            PackageState::Uninstalled => write!(f, "uninstalled"),
            PackageState::Disabled => write!(f, "disabled"),
        }
    }
}

impl FromStr for PackageState {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lowercased = s.to_ascii_lowercase();
        match lowercased.as_str() {
            "enabled" => Ok(Self::Enabled),
            "uninstalled" => Ok(Self::Uninstalled),
            "disabled" => Ok(Self::Disabled),
            _ => Err(anyhow!("unknown package state {}", lowercased)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]

pub struct Package {
    name: String,
    state: PackageState,
    ptype: String,
}

pub trait ListPackages {
    fn list_packages(&self, device_id: String, user_id: String) -> Result<Vec<Package>>;
}

const LIST_ALL_PACKAGES_INCLUDING_UNINSTALLED: &str = "pm list packages -u";
const LIST_SYSTEM_PACKAGES: &str = "pm list packages -s";
const LIST_THIRD_PARTY_PACKAGES: &str = "pm list packages -3";
const LIST_ENABLED_PACKAGES: &str = "pm list packages -e";
const LIST_DISABLED_PACKAGES: &str = "pm list packages -d";

pub struct ADBTerminalImpl {}

impl ADBTerminalImpl {
    pub fn list_packages(&self, device_id: String, user_id: String) -> Result<Vec<Package>> {
        let (mut all_pkg, mut enabled_pkg, mut disabled_pkg, mut sys_pkg, mut tpp_pkg): (
            HashSet<String>,
            HashSet<String>,
            HashSet<String>,
            HashSet<String>,
            HashSet<String>,
        ) = (
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
        );

        let (cmd_all_pkg, cmd_enabled_pkg, cmd_disabled_pkg, cmd_system_pkg, cmd_tpp_pkg) = (
            ADBShell::new_for_device(
                device_id.to_owned(),
                &[LIST_ALL_PACKAGES_INCLUDING_UNINSTALLED],
            ),
            ADBShell::new_for_device(
                device_id.to_owned(),
                &[LIST_ENABLED_PACKAGES, "--user", &user_id],
            ),
            ADBShell::new_for_device(
                device_id.to_owned(),
                &[LIST_DISABLED_PACKAGES, "--user", &user_id],
            ),
            ADBShell::new_for_device(
                device_id.to_owned(),
                &[LIST_SYSTEM_PACKAGES, "--user", &user_id],
            ),
            ADBShell::new_for_device(
                device_id.to_owned(),
                &[LIST_THIRD_PARTY_PACKAGES, "--user", &user_id],
            ),
        );

        let res = Self::_execute_and_parse(cmd_all_pkg, &mut all_pkg)
            .and_then(|_| Self::_execute_and_parse(cmd_enabled_pkg, &mut enabled_pkg))
            .and_then(|_| Self::_execute_and_parse(cmd_disabled_pkg, &mut disabled_pkg))
            .and_then(|_| Self::_execute_and_parse(cmd_system_pkg, &mut sys_pkg))
            .and_then(|_| Self::_execute_and_parse(cmd_tpp_pkg, &mut tpp_pkg));

        match res {
            Err(e) => {
                return Err(e.into());
            }
            Ok(_) => {}
        }

        let mut pkgs: Vec<Package> = vec![];

        for pname in all_pkg.iter() {
            let mut pstate = PackageState::Uninstalled; //todo: set this as unknown, we cant assume uninstalled

            if enabled_pkg.contains(pname) {
                pstate = PackageState::Enabled;
            } else if disabled_pkg.contains(pname) {
                pstate = PackageState::Disabled;
            }

            let mut ptype = "";
            if sys_pkg.contains(pname) {
                ptype = "system"
            } else if tpp_pkg.contains(pname) {
                ptype = "thirdparty"
            }

            pkgs.push(Package { name: pname.to_string(), state: pstate, ptype: ptype.to_string() })
        }

        return Ok(pkgs);
    }

    fn _execute_and_parse(cmd: ADBShell, container: &mut HashSet<String>) -> Result<()> {
        let res = cmd.execute();
        match res {
            Err(e) => {
                return Err(e.into());
            }
            Ok(o) => {
                let ot = o.replace("package:", "");
                for l in ot.lines() {
                    container.insert(l.to_string());
                }
                return Ok(());
            }
        }
    }
}
