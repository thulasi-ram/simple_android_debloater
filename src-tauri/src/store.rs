use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{devices::Device, packages::Package, users::User, DeviceWithUsers};

#[derive(Debug, Clone)]
pub struct UserWithPackage {
    user: User,
    packages_map: HashMap<String, Package>,
}

impl UserWithPackage {
    pub fn add_package(&mut self, p: Package) {
        self.packages_map.insert(p.name.to_owned(), p);
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

    pub fn user(&mut self, user_id: String) -> Option<&mut UserWithPackage> {
        return self.users_map.get_mut(&user_id);
    }
}

pub type Store = HashMap<String, DeviceWithUserPackages>;
