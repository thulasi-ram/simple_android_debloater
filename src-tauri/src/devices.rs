use crate::adb_cmd::{ADBCommand, ADBRaw, ADBShell};
use anyhow::{anyhow, Error, Result};
use core::result::Result::Ok;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DeviceState {
    /// The device is not connected to adb or is not responding.
    Offline,
    /// The device is now connected to the adb server. Note that this state does not imply that the Android system is fully booted and operational because the device connects to adb while the system is still booting. However, after boot-up, this is the normal operational state of an device.
    Device,
    /// There is no device connected.
    NoDevice,
    /// The device is unauthorized.
    Unauthorized,
}

impl Display for DeviceState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeviceState::Offline => write!(f, "offline"),
            DeviceState::Device => write!(f, "device"),
            DeviceState::NoDevice => write!(f, "no device"),
            DeviceState::Unauthorized => write!(f, "unauthorized"),
        }
    }
}

impl FromStr for DeviceState {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lowercased = s.to_ascii_lowercase();
        match lowercased.as_str() {
            "offline" => Ok(Self::Offline),
            "device" => Ok(Self::Device),
            "no device" => Ok(Self::NoDevice),
            "unauthorized" => Ok(Self::Unauthorized),
            _ => Err(anyhow!("unknown device state {}", lowercased)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Device {
    pub id: String,
    pub state: DeviceState,
    pub make: String,
    pub model: String,
    pub name: String,
}

pub trait ListDevices {
    fn list_devices(&self) -> Result<Vec<Device>>;
}

pub struct ADBTerminalImpl {}

impl ADBTerminalImpl {
    pub fn list_devices(&self) -> Result<Vec<Device>> {
        let res = ADBRaw::new(&["devices"]).execute();
        match res {
            Err(e) => {
                return Err(e.into());
            }
            Ok(o) => {
                let ot = o.replace("List of devices attached", "");
                let ots = ot.trim();

                let devices: Result<Vec<_>> = ots
                    .lines()
                    .map(|s| Self::_parse_device(s))
                    .map(|d| Self::_set_prop(d))
                    .collect();

                return devices;
            }
        }
    }

    fn _parse_device(s: &str) -> Result<Device> {
        let ss: Vec<&str> = s.split_whitespace().collect();
        if ss.len() < 2 {
            return Err(anyhow!("unable to parse device. input {}", s));
        }
        return Ok(Device {
            id: ss[0].to_string(),
            state: DeviceState::from_str(ss[1]).unwrap(),
            make: String::from(""),
            model: String::from(""),
            name: String::from(""),
        });
    }

    fn _set_prop(device: Result<Device>) -> Result<Device> {
        match device {
            Err(e) => {
                return Err(e);
            }
            Ok(d) => {
                let res = ADBShell::new_for_device(d.id.to_owned(), &["getprop"]).execute();
                match res {
                    Err(e) => {
                        return Err(e.into());
                    }
                    Ok(o) => {

                        // helper to extract the prop when a match is found
                        let parse_val = |v: &str| {
                            let split = v.split_once(":");

                            match split {
                                Some(s) => {
                                    return s
                                        .1
                                        .trim()
                                        .trim_start_matches("[")
                                        .trim_end_matches("]")
                                        .to_string()
                                }
                                None => {
                                    return String::from("parse err");
                                }
                            }
                        };

                        let (mut make, mut model, mut name) =
                        (String::from(""), String::from(""), String::from(""));

                        for l in o.lines() {
                            match l {
                                s if s.contains("ro.product.product.brand") => {
                                    make = parse_val(s);
                                }
                                s if s.contains("ro.product.model") => {
                                    model = parse_val(s);
                                }
                                s if s.contains("ro.product.odm.marketname") => {
                                    name = parse_val(s);
                                }
                                _ => (),
                            }
                        }

                        return Ok(Device {
                            id: d.id.to_owned(),
                            state: d.state,
                            make,
                            model,
                            name,
                        });
                    }
                }
            }
        }
    }
}
