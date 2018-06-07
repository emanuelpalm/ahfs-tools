use std::process;

pub fn exit_after<F>(f: F) -> !
    where F: FnOnce() -> Result<(), ::error::Error>,
{
    process::exit(match f() {
        Ok(()) => 0,
        Err(err) => {
            println!("{}", err);
            1
        }
    })
}