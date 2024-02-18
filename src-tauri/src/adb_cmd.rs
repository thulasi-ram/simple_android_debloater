use std::process::Command;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

use log::info;

pub trait ADBCommand: Sized {
    fn execute(&self) -> Result<String, ADBError>;

    fn arg<S: AsRef<str>>(self, arg: S) -> Self;
    fn args<I, S>(self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut s1 = self;
        for arg in args {
            s1 = s1.arg(arg);
        }
        s1
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ADBError {
    #[error("ADB Error {0}")]
    Unknown(String),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ADBRaw {
    cmd_str: String,
    argsv: Vec<String>,
}

impl ADBRaw {
    pub fn new(adb_path: String) -> Self {
        let mut cmd_str = "adb";
        if !adb_path.is_empty() {
            cmd_str = adb_path.as_str();
        }
        Self {
            cmd_str: cmd_str.to_string(),
            argsv: vec![],
        }
    }
    
}

impl ADBCommand for ADBRaw {
    fn arg<S: AsRef<str>>(self, arg: S) -> Self {
        // https://users.rust-lang.org/t/best-way-to-clone-and-append-a-single-element/68675/2

        let mut s1 = self;
        s1.argsv.push(arg.as_ref().to_owned());
        return s1;
    }

    fn execute(&self) -> Result<String, ADBError> {
        let mut command = Command::new(self.cmd_str.to_owned());
        command.args(self.argsv.to_vec());

        // let args = self
        //     .sub_commands
        //     .iter()
        //     .map(|s| s.as_str())
        //     .collect::<Vec<&str>>();
        // command.args(args);

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

#[derive(Debug, Clone)]
pub struct ADBShell {
    adb_raw: ADBRaw,
}

impl ADBShell {
    pub fn new(adb_path: String) -> Self {
        let adbr = ADBRaw::new(adb_path).arg("shell");
        Self { adb_raw: adbr }
    }

    pub fn for_device(self, device_id: String) -> Self {
        let mut s1 = self;
        s1.adb_raw = s1.adb_raw.args(vec!["-s", &device_id]);
        return s1;
    }
}

impl ADBCommand for ADBShell {
    fn arg<S: AsRef<str>>(self, arg: S) -> Self {
        let mut s1 = self;
        s1.adb_raw = s1.adb_raw.arg(arg.as_ref());
        return s1;
    }

    fn execute(&self) -> Result<String, ADBError> {
        return self.adb_raw.execute();
    }
}
