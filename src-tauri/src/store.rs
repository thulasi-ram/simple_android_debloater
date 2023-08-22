use std::collections::HashMap;

use crate::{devices::Device, packages::Package, users::User, DeviceWithUsers};
use anyhow::{anyhow, Result};

#[derive(Debug, Clone)]
pub struct UserWithPackage {
    user: User,
    packages_map: HashMap<String, Package>,
}

impl UserWithPackage {
    pub fn add_package(&mut self, p: Package) {
        self.packages_map.insert(p.name.to_owned(), p);
    }

    pub fn get_package(&mut self, p: &str) -> Option<&mut Package> {
        self.packages_map.get_mut(p)
    }
}

#[derive(Debug, Clone)]
pub struct DeviceWithUserPackages {
    device: Device,
    users_map: HashMap<String, UserWithPackage>,
}

impl DeviceWithUserPackages {
    pub fn new_from_device_with_users(du: DeviceWithUsers) -> Self {
        let mut users_map: HashMap<String, UserWithPackage> = HashMap::new();
        for user in du.users {
            users_map.insert(
                user.id.to_owned(),
                UserWithPackage {
                    user: user,
                    packages_map: HashMap::new(),
                },
            );
        }

        return Self {
            device: du.device,
            users_map: users_map,
        };
    }

    pub fn user(&mut self, user_id: String) -> Result<&mut UserWithPackage> {
        let user = self
            .users_map
            .get_mut(&user_id)
            .ok_or(anyhow!("user is invalid"))?;

        return Ok(user);
    }
}

#[derive(Debug, Clone)]
pub struct Store(HashMap<String, DeviceWithUserPackages>);

impl Store {
    pub fn new() -> Store {
        Store(HashMap::new())
    }

    pub fn device(&mut self, device_id: String) -> Result<&mut DeviceWithUserPackages> {
        let device = self
            .0
            .get_mut(&device_id)
            .ok_or(anyhow!("device is invalid"))?;

        return Ok(device);
    }

    pub fn insert_device_with_user(&mut self, du: DeviceWithUsers) {
        let mut new_dup = DeviceWithUserPackages::new_from_device_with_users(du.clone());
        let existing_dup = self.0.get(&du.device.id.to_owned());
        match existing_dup {
            Some(dup) => {
                for (user_id, uwp) in &dup.users_map {
                    let new_uwp = new_dup.user(user_id.to_string());
                    match new_uwp {
                        Ok(nuwp) => {
                            for (_, p) in &uwp.packages_map {
                                nuwp.add_package(p.clone())
                            }
                        },
                        Err(_) => {}
                    }
                }
            }
            None => {
                self.0.insert(du.device.id.to_owned(), new_dup);
                return;
            }
        }
        return;
    }
}
