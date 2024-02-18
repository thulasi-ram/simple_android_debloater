use crate::adb_cmd::{ADBCommand, ADBShell};
use anyhow::{anyhow, Error, Result};
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PackageState {
    Enabled,
    Uninstalled,
    Disabled,
    Hidden,
}

impl Display for PackageState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PackageState::Enabled => write!(f, "enabled"),
            PackageState::Uninstalled => write!(f, "uninstalled"),
            PackageState::Disabled => write!(f, "disabled"),
            PackageState::Hidden => write!(f, "hidden"),
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
            "hidden" => Ok(Self::Hidden),
            _ => Err(anyhow!("unknown package state {}", lowercased)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PackageType {
    System,
    ThirdParty,
    Unknown,
}

impl Display for PackageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PackageType::System => write!(f, "system"),
            PackageType::ThirdParty => write!(f, "thirdparty"),
            PackageType::Unknown => write!(f, "unknown"),
        }
    }
}

impl FromStr for PackageType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lowercased = s.to_ascii_lowercase();
        match lowercased.as_str() {
            "system" => Ok(Self::System),
            "thirdparty" => Ok(Self::ThirdParty),
            "unknown" => Ok(Self::Unknown),
            _ => Err(anyhow!("unsupported package type {}", lowercased)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Package {
    pub name: String,
    state: PackageState,
    ptype: PackageType,
    package_prefix: String,
}

impl Package {
    pub fn set_state(&mut self, s: PackageState) {
        self.state = s;
    }
}

pub trait ListPackages {
    fn list_packages(&self, device_id: String, user_id: String) -> Result<Vec<Package>>;
}

pub trait DisablePackage {
    fn disable_package(
        &self,
        device_id: String,
        user_id: String,
        pkg: String,
        clear_pkg: bool,
    ) -> Result<()>;
}

pub trait EnablePackage {
    fn enable_package(&self, device_id: String, user_id: String, pkg: String) -> Result<()>;
}

const LIST_ALL_PACKAGES_INCLUDING_UNINSTALLED: &str = "pm list packages -u -f";
const LIST_SYSTEM_PACKAGES: &str = "pm list packages -u -s";
const LIST_THIRD_PARTY_PACKAGES: &str = "pm list packages -u -3";
const LIST_ENABLED_PACKAGES: &str = "pm list packages -e";
const LIST_DISABLED_PACKAGES: &str = "pm list packages -d";
const LIST_UNINSTALLED_DISABLED_PACKAGES: &str = "pm list packages -u -d";

pub struct ADBTerminalImpl<F>
where
    F: ADBCommand + ?Sized,
{
    adb_command: F,
}

impl ADBTerminalImpl<ADBShell> {
    pub fn new(adb_path: String) -> Self {
        Self {
            adb_command: ADBShell::new(adb_path),
        }
    }
}

struct PackageAttribs {
    package_path: String,
}

impl PackageAttribs {
    fn package_prefix(&self) -> String {
        let splits: Vec<&str> = self.package_path.splitn(4, "/").collect();
        return splits[..3].join("/");
    }
}

lazy_static! {
    static ref PACKAGE_PARSE_REGEX: Regex = Regex::new(r"(.*)\.apk\=(.*)").unwrap();
}

impl ADBTerminalImpl<ADBShell> {
    pub fn list_packages(&self, device_id: String, user_id: String) -> Result<Vec<Package>> {
        let shell_cmd = self.adb_command.clone().for_device(device_id.to_owned());

        let (
            mut all_pkg,
            mut enabled_pkg,
            mut disabled_pkg,
            mut sys_pkg,
            mut tpp_pkg,
            mut uninstalled_disabled_pkg,
        ): (
            HashMap<String, PackageAttribs>,
            HashSet<String>,
            HashSet<String>,
            HashSet<String>,
            HashSet<String>,
            HashSet<String>,
        ) = (
            HashMap::new(),
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
        );

        let (
            cmd_enabled_pkg,
            cmd_disabled_pkg,
            cmd_system_pkg,
            cmd_tpp_pkg,
            cmd_uninstalled_disabled_pkg,
        ) = (
            shell_cmd
                .clone()
                .args(&[LIST_ENABLED_PACKAGES, "--user", &user_id]),
            shell_cmd
                .clone()
                .args(&[LIST_DISABLED_PACKAGES, "--user", &user_id]),
            shell_cmd
                .clone()
                .args(&[LIST_SYSTEM_PACKAGES, "--user", &user_id]),
            shell_cmd
                .clone()
                .args(&[LIST_THIRD_PARTY_PACKAGES, "--user", &user_id]),
            shell_cmd
                .clone()
                .args(&[LIST_UNINSTALLED_DISABLED_PACKAGES, "--user", &user_id]),
        );

        fn callback(container: &mut HashSet<String>) -> impl FnMut(String) -> Result<()> + '_ {
            let parser = |s: String| -> Result<()> {
                let ot = s.replace("package:", "");
                for l in ot.lines() {
                    container.insert(l.to_string());
                }
                return Ok(());
            };
            return parser;
        }

        let res = Self::execute_list_all_with_fallback(shell_cmd, user_id.to_owned(), &mut all_pkg)
            .and_then(|_| Self::_execute_and_parse(cmd_enabled_pkg, callback(&mut enabled_pkg)))
            .and_then(|_| Self::_execute_and_parse(cmd_disabled_pkg, callback(&mut disabled_pkg)))
            .and_then(|_| Self::_execute_and_parse(cmd_system_pkg, callback(&mut sys_pkg)))
            .and_then(|_| Self::_execute_and_parse(cmd_tpp_pkg, callback(&mut tpp_pkg)))
            .and_then(|_| {
                Self::_execute_and_parse(
                    cmd_uninstalled_disabled_pkg,
                    callback(&mut uninstalled_disabled_pkg),
                )
            });

        match res {
            Err(e) => {
                return Err(e.into());
            }
            Ok(_) => {}
        }

        let mut pkgs: Vec<Package> = vec![];

        for (pname, pattrib) in all_pkg.iter() {
            let mut pstate = PackageState::Hidden;

            if enabled_pkg.contains(pname) {
                pstate = PackageState::Enabled;
            } else if disabled_pkg.contains(pname) {
                pstate = PackageState::Disabled;
            } else if uninstalled_disabled_pkg.contains(pname) {
                pstate = PackageState::Uninstalled
            }

            let mut ptype: PackageType = PackageType::Unknown;
            if sys_pkg.contains(pname) {
                ptype = PackageType::System
            } else if tpp_pkg.contains(pname) {
                ptype = PackageType::ThirdParty
            }

            pkgs.push(Package {
                name: pname.to_string(),
                state: pstate,
                ptype: ptype,
                package_prefix: pattrib.package_prefix(),
            })
        }

        pkgs.sort_by(|a, b| a.name.cmp(&b.name));

        return Ok(pkgs);
    }

    pub fn disable_package(
        &self,
        device_id: String,
        user_id: String,
        pkg: String,
        clear_pkg: bool,
    ) -> Result<()> {
        let shell_cmd: ADBShell = self.adb_command.clone().for_device(device_id);

        let (cmd_disable_pkg, cmd_fstop_pkg, cmd_clear_pkg) = (
            shell_cmd
                .clone()
                .args(["pm disable-user", "--user", &user_id, &pkg.to_owned()]),
            shell_cmd
                .clone()
                .args(["am force-stop", "--user", &user_id, &pkg.to_owned()]),
            shell_cmd
                .clone()
                .args(&["pm clear", "--user", &user_id, &pkg.to_owned()]),
        );

        Self::_execute_and_parse(cmd_disable_pkg, |s| {
            if s.contains(&format!(
                "Package {} new state: disabled-user",
                pkg.to_owned()
            )) {
                return Ok(());
            }
            return Err(anyhow!(s));
        })
        .and_then(|_| {
            Self::_execute_and_parse(cmd_fstop_pkg, |s| {
                if s.is_empty() {
                    return Ok(());
                }
                return Err(anyhow!(s));
            })
        })?;

        if clear_pkg {
            Self::_execute_and_parse(cmd_clear_pkg, |s| {
                if s.eq("Success") {
                    return Ok(());
                }
                return Err(anyhow!(s));
            })?
        }

        return Ok(());
    }

    pub fn enable_package(&self, device_id: String, user_id: String, pkg: String) -> Result<()> {
        let shell_cmd: ADBShell = self.adb_command.clone().for_device(device_id);

        let cmd_enable_pkg = shell_cmd.args(["pm enable", "--user", &user_id, &pkg.to_owned()]);

        Self::_execute_and_parse(cmd_enable_pkg, |s| {
            if s.contains(&format!("Package {} new state: enabled", pkg.to_owned())) {
                return Ok(());
            }
            return Err(anyhow!(s));
        })?;

        return Ok(());
    }

    pub fn install_package(&self, device_id: String, user_id: String, pkg: String) -> Result<()> {
        let shell_cmd: ADBShell = self.adb_command.clone().for_device(device_id);

        let cmd_enable_pkg = shell_cmd.args([
            "cmd package install-existing",
            "--user",
            &user_id,
            &pkg.to_owned(),
        ]);

        Self::_execute_and_parse(cmd_enable_pkg, |s| {
            if s.contains(&format!("Package {} new state: enabled", pkg.to_owned())) {
                return Ok(());
            }
            return Err(anyhow!(s));
        })?;

        return Ok(());
    }

    fn execute_list_all_with_fallback(
        shell_cmd: ADBShell,
        user_id: String,
        mut container: &mut HashMap<String, PackageAttribs>,
    ) -> Result<()> {
        let (cmd_all_pkg, cmd_all_package_user0_fallback, cmd_all_package_user_fallback) = (
            shell_cmd
                .clone()
                .args([LIST_ALL_PACKAGES_INCLUDING_UNINSTALLED]),
            shell_cmd
                .clone()
                .args([LIST_ALL_PACKAGES_INCLUDING_UNINSTALLED, "--user", "0"]),
            shell_cmd
                .clone()
                .args([LIST_ALL_PACKAGES_INCLUDING_UNINSTALLED, "--user", &user_id]),
        );

        fn callback(
            container: &mut HashMap<String, PackageAttribs>,
        ) -> impl FnMut(String) -> Result<()> + '_ {
            let parser = |s: String| -> Result<()> {
                let ot = s.replace("package:", "");
                for l in ot.lines() {
                    let caps = PACKAGE_PARSE_REGEX.captures(l).unwrap();
                    container.insert(
                        caps.get(2).unwrap().as_str().to_string(),
                        PackageAttribs {
                            package_path: caps.get(1).unwrap().as_str().to_string(),
                        },
                    );
                }
                return Ok(());
            };
            return parser;
        }

        return Self::_execute_and_parse(cmd_all_pkg, callback(&mut container))
            .or_else(|e| {
                if !e
                    .to_string()
                    .contains("Shell does not have permission to access user")
                {
                    return Err(e);
                }
                Self::_execute_and_parse(cmd_all_package_user0_fallback, callback(&mut container))
            })
            .or_else(|e| {
                if !e
                    .to_string()
                    .contains("Shell does not have permission to access user")
                {
                    return Err(e);
                }
                Self::_execute_and_parse(cmd_all_package_user_fallback, callback(&mut container))
            });
    }

    fn _execute_and_parse(
        cmd: ADBShell,
        mut parser: impl FnMut(String) -> Result<()>,
    ) -> Result<()> {
        let res = cmd.execute();
        match res {
            Err(e) => {
                return Err(e.into());
            }
            Ok(o) => {
                return parser(o);
            }
        }
    }
}
