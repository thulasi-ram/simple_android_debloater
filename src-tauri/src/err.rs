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
                eprintln!("{}: {:?}", msg, e);
                None
            }
        }
    }
}