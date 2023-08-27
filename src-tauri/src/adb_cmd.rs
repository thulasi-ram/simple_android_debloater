use std::process::Command;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

use log::info;

pub trait ADBCommand {
    fn execute(&self) -> Result<String, ADBError>;
}

#[derive(Debug, thiserror::Error)]
pub enum ADBError {
    #[error("ADB Error {0}")]
    Unknown(String),
}

#[derive(Debug, Clone)]
pub struct ADBRaw {
    adb_path: String,
    sub_commands: Vec<String>,
}

impl ADBRaw {
    pub fn new(adb_path: String, value: Vec<String>) -> Self {
        Self {
            adb_path: adb_path,
            sub_commands: value,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ADBShell {
    adb_path: String,
    sub_commands: Vec<String>,
    device_id: String,
}

impl ADBShell {
    pub fn new(adb_path: String) -> Self {
        Self {
            adb_path: adb_path,
            sub_commands: vec![],
            device_id: String::from(""),
        }
    }

    pub fn for_device(mut self, device_id: String) -> Self {
        self.device_id = device_id;
        return self;
    }

    pub fn with_commands(mut self, sub_commands: &[&str]) -> Self {
        self.sub_commands = sub_commands.iter().map(|s| String::from(*s)).collect();
        return self;
    }
}

impl ADBCommand for ADBRaw {
    fn execute(&self) -> Result<String, ADBError> {

        let mut cmd_str = "adb";
        if !self.adb_path.is_empty() {
            cmd_str = self.adb_path.as_str();
        }

        let mut command = Command::new(cmd_str);

        // https://stackoverflow.com/a/38186733/6323666
        let args = self
            .sub_commands
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<&str>>();
        command.args(args);

        info!("command {:?}", command);

        #[cfg(target_os = "windows")]
        let command = command.creation_flags(0x08000000); // do not open a cmd window

        match command.output() {
            Err(e) => Err(ADBError::Unknown(e.to_string())),
            Ok(o) => {
                if o.status.success() {
                    Ok(String::from_utf8(o.stdout)
                        .map_err(|e| ADBError::Unknown(e.to_string()))?
                        .trim_end()
                        .to_string())
                } else {
                    let stdout = String::from_utf8(o.stdout)
                        .map_err(|e| ADBError::Unknown(e.to_string()))?
                        .trim_end()
                        .to_string();
                    let stderr = String::from_utf8(o.stderr)
                        .map_err(|e| ADBError::Unknown(e.to_string()))?
                        .trim_end()
                        .to_string();

                    // ADB does really weird things. Some errors are not redirected to stderr
                    let err = if stdout.is_empty() { stderr } else { stdout };
                    Err(ADBError::Unknown(err.to_string()))
                }
            }
        }
    }
}

impl ADBCommand for ADBShell {
    fn execute(&self) -> Result<String, ADBError> {
        let mut sub_commands_with_shell: Vec<String> = vec![String::from("shell")];

        if !String::is_empty(&self.device_id.to_owned()) {
            sub_commands_with_shell.insert(0, String::from("-s"));
            sub_commands_with_shell.insert(1, self.device_id.to_owned());
        }

        sub_commands_with_shell.extend(self.sub_commands.to_owned());
        let adb_raw = ADBRaw::new(self.adb_path.to_owned(), sub_commands_with_shell);
        return adb_raw.execute();
    }
}
