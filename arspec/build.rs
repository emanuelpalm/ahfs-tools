use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let path_out = env::var("OUT_DIR").unwrap();
    let path_meta = Path::new(&path_out).join("meta.rs");
    let mut file_meta = File::create(&path_meta).unwrap();

    write!(file_meta, concat!(
        "/// Major version of the ARSPEC library.\n",
        "#[allow(dead_code)]\n",
        "pub const VERSION_MAJOR: usize = {0};\n",
        "\n",
        "/// Minor version of the ARSPEC library.\n",
        "#[allow(dead_code)]\n",
        "pub const VERSION_MINOR: usize = {1};\n",
        "\n",
        "/// Patch version of the ARSPEC library.\n",
        "#[allow(dead_code)]\n",
        "pub const VERSION_PATCH: usize = {2};\n",
        "\n",
        "/// Full version of the ARSPEC library.\n",
        "#[allow(dead_code)]\n",
        "pub const VERSION_STR: &'static str = \"{0}.{1}.{2}\";\n"),
           env::var("CARGO_PKG_VERSION_MAJOR").unwrap(),
           env::var("CARGO_PKG_VERSION_MINOR").unwrap(),
           env::var("CARGO_PKG_VERSION_PATCH").unwrap())
        .unwrap();
}