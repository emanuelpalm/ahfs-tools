/// Adds color codes before and after given string literal.
#[cfg(feature = "vt100")]
#[macro_export]
macro_rules! color {
    (b: $str:expr) => (concat!("\x1b[34m", $str, "\x1b[0m"));
    (c: $str:expr) => (concat!("\x1b[36m", $str, "\x1b[0m"));
    (g: $str:expr) => (concat!("\x1b[32m", $str, "\x1b[0m"));
    (m: $str:expr) => (concat!("\x1b[35m", $str, "\x1b[0m"));
    (_: $str:expr) => ($str);
    (r: $str:expr) => (concat!("\x1b[31m", $str, "\x1b[0m"));
    (y: $str:expr) => (concat!("\x1b[33m", $str, "\x1b[0m"));
}

#[cfg(not(feature = "vt100"))]
#[macro_export]
macro_rules! color {
    ($_:tt : $str:expr) => ($str);
}
