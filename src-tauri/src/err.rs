use log::error;

use crate::sad::SADError;
use anyhow::anyhow;

pub trait ResultOkPrintErrExt<T> {
    fn ok_or_print_err(self, msg: &str) -> Option<T>;
}

impl<T, E> ResultOkPrintErrExt<T> for Result<T, E>
where
    E: ::std::fmt::Debug,
{
    fn ok_or_print_err(self, msg: &str) -> Option<T> {
        match self {
            Ok(v) => Some(v),
            Err(e) => {
                error!("{}: {:?}", msg, e);
                None
            }
        }
    }
}

pub trait IntoSADError<T> {
    fn into_sad_error(self, msg: &str) -> Result<T, SADError>;
}

impl<T, E> IntoSADError<T> for Result<T, E>
where
    E: ::std::fmt::Display,
{
    fn into_sad_error(self, msg: &str) -> Result<T, SADError> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => Err(anyhow!("{}: {}", msg, e.to_string()).into()),
        }
    }
}
