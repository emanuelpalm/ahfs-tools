use std::process;

/// Executes `f`, and then exits application after it returns.
///
/// If `f` returns an error, the error is printed to console and 1 is used as
/// exit code.
pub fn exit_after<F>(f: F) -> !
    where F: FnOnce() -> Result<(), Box<::ErrorCode>>,
{
    process::exit(match f() {
        Ok(()) => 0,
        Err(err) => {
            println!("{}", ::format_error(err.as_ref()).unwrap());
            1
        }
    })
}