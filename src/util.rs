#[macro_export]
macro_rules! errorln(
    ($($arg:tt)*) => { {
        let r = writeln!(&mut ::std::io::stderr(), $($arg)*);
        r.expect("failed printing to stderr");
    } }
);

// Note:
// string::String::insert_str() is not available because it's unstable.
//   https://github.com/rust-lang/rust/issues/35553
pub fn insert(target: &mut String, index: usize, replaced: &str) {
    for c in replaced.chars().rev() {
        target.insert(index, c);
    }
}
