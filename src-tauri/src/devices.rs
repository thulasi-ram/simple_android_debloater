use crate::adb_cmd::{ADBCommand, ADBRaw};
use anyhow::{anyhow, Error, Result};
use core::result::Result::Ok;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Device {
    pub identifier: String,
    pub state: DeviceState,
    pub make: String,
    pub model: String,
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

                let devices: Result<Vec<_>> = ots.lines().map(|s| Self::_parse_device(s)).collect();

                return devices;
            }
        }
    }

    fn _parse_device(s: &str) -> Result<Device> {
        let ss: Vec<&str> = s.split_whitespace().collect();
        return Ok(Device {
            identifier: ss[0].to_string(),
            state: DeviceState::from_str(ss[1]).unwrap(),
            make: String::from(""),
            model: String::from(""),
        });
    }
}
