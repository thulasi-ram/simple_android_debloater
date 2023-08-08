use std::process::Command;

pub trait ADBCommand {
    fn execute(&self) -> Result<String, ADBError>;
}

#[derive(Debug, thiserror::Error)]
pub enum ADBError {
    #[error("ADB Error {0}")]
    Unknown(String),
}

pub struct ADBRaw {
    sub_commands: Vec<String>,
}

impl ADBRaw {
    pub fn new(value: &[&str]) -> Self {
        Self {
            sub_commands: value.iter().map(|s| String::from(*s)).collect(),
        }
    }
}

pub struct ADBShell {
    sub_commands: Vec<String>,
    device_id: String,
}

impl ADBShell {
    pub fn new(value: &[&str]) -> Self {
        Self {
            sub_commands: value.iter().map(|s| String::from(*s)).collect(),
            device_id: String::from(""),
        }
    }
    pub fn new_for_device(device_id: String, value: &[&str]) -> Self {
        Self {
            sub_commands: value.iter().map(|s| String::from(*s)).collect(),
            device_id: device_id,
        }
    }
}

impl ADBCommand for ADBRaw {
    fn execute(&self) -> Result<String, ADBError> {
        let mut command = Command::new("adb");

        // https://stackoverflow.com/a/38186733/6323666
        let args = self
            .sub_commands
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<&str>>();
        command.args(args);

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
            sub_commands_with_shell.insert(1, String::from(self.device_id.to_owned()));
        }

        sub_commands_with_shell.extend(self.sub_commands.to_owned());
        let adb_raw = ADBRaw {
            sub_commands: sub_commands_with_shell,
        };
        return adb_raw.execute();
    }
}
