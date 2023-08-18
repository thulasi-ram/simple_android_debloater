use crate::{packages::Package, DeviceWithUsers};
use anyhow::{anyhow, Error, Ok, Result};
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};

use std::{
    fmt::Display,
    str::FromStr,
    sync::atomic::{AtomicUsize, Ordering},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EventType {
    DeviceEvent,
    PackageEvent,
}

impl Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventType::DeviceEvent => write!(f, "device_event"),
            EventType::PackageEvent => write!(f, "package_event"),
        }
    }
}

impl FromStr for EventType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lowercased = s.to_ascii_lowercase();
        match lowercased.as_str() {
            "device_event" => Ok(Self::DeviceEvent),
            "package_event" => Ok(Self::PackageEvent),
            _ => Err(anyhow!("unknown device state {}", lowercased)),
        }
    }
}

pub trait Event {
    fn eid(self: &Self) -> String;
    fn etype(self: &Self) -> EventType;
    fn epayload(self: &Self) -> Result<String>;
}

static COUNTER: AtomicUsize = AtomicUsize::new(1);

fn get_id() -> String {
    let us = COUNTER.fetch_add(1, Ordering::Relaxed);
    return us.to_string();
}

#[derive(Debug, Clone)]
pub struct DeviceEvent {
    eid: String,
    etype: EventType,
    device: DeviceWithUsers,
}

impl DeviceEvent {
    pub fn new(d: DeviceWithUsers) -> Self {
        Self {
            eid: get_id(),
            etype: EventType::DeviceEvent,
            device: d,
        }
    }
}

impl Event for DeviceEvent {
    fn eid(&self) -> String {
        self.eid.to_owned()
    }

    fn etype(&self) -> EventType {
        self.etype.to_owned()
    }

    fn epayload(&self) -> Result<String> {
        let res = serde_json::to_string(&self.device)?;
        return Ok(res);
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct DeviceUserPackage {
    device_id: String,
    user_id: String,
    package: Package,
}

#[derive(Debug, Clone)]
pub struct PackageEvent {
    eid: String,
    etype: EventType,
    package: DeviceUserPackage,
}

impl PackageEvent {
    pub fn new(device_id: String, user_id: String, package: Package) -> Self {
        Self {
            eid: get_id(),
            etype: EventType::PackageEvent,
            package: DeviceUserPackage {
                device_id,
                user_id,
                package,
            },
        }
    }
}

impl Event for PackageEvent {
    fn eid(&self) -> String {
        self.eid.to_owned()
    }

    fn etype(&self) -> EventType {
        self.etype.to_owned()
    }

    fn epayload(&self) -> Result<String> {
        let res = serde_json::to_string(&self.package)?;
        return Ok(res);
    }
}

pub type AsyncEvent = Box<dyn Event + Send + 'static>;
