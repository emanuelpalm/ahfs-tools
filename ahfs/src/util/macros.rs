/// Adds VT100 color code before and after given string literal.
macro_rules! str_color {
    (blue: $str:expr) => (concat!("\x1b[34m", $str, "\x1b[0m"));
    (none: $str:expr) => ($str);
    ( red: $str:expr) => (concat!("\x1b[31m", $str, "\x1b[0m"));
}