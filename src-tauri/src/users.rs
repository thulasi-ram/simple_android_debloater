use crate::adb_cmd::{ADBCommand, ADBShell};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use tauri::regex::Regex;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub name: String,
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "user {}", self.id)
    }
}

pub trait ListUsers {
    fn list_users(&self, device_id: String) -> Result<Vec<User>>;
}

pub struct ADBTerminalImpl {}

impl ADBTerminalImpl {
    pub fn list_users(&self, device_id: String) -> Result<Vec<User>> {
        let res = ADBShell::new_for_device(device_id, &["pm list users "]).execute();
        match res {
            Err(e) => {
                return Err(e.into());
            }
            Ok(o) => {
                let re = Regex::new(r"UserInfo\{(.*)\}").unwrap();

                let mut users: Vec<User> = vec![];
                for (_, [cap]) in re.captures_iter(&o).map(|c| c.extract()) {
                    let split: Vec<&str> = cap.split(":").collect();
                    if split.len() < 2 {
                        return Err(anyhow!("unable to parse user. input {}", cap));
                    }
                    users.push(User {
                        id: split[0].to_string(),
                        name: split[1].to_string(),
                    })
                }
                return Ok(users);
            }
        }
    }
}
