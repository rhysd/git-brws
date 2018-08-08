#[macro_export]
macro_rules! errorln(
    ($($arg:tt)*) => { {
        let r = writeln!(&mut ::std::io::stderr(), $($arg)*);
        r.expect("failed printing to stderr");
    } }
);

pub type ErrorMsg = String;
pub type Result<T> = ::std::result::Result<T, ErrorMsg>;
