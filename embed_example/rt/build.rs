use std::{env, error::Error, fs::File, io::Write, path::PathBuf};

extern crate cc;

fn main() -> Result<(), Box<dyn Error>> {
    let target = env::var("TARGET").unwrap();

    if target.starts_with("thumbv") {
        cc::Build::new().file("asm.s").compile("asm");
    }

    // build directory for this crate
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    // extend the library search path
    println!("cargo:rustc-link-search={}", out_dir.display());

    // put `link.x` in the build directory
    File::create(out_dir.join("link.x"))?.write_all(include_bytes!("link.x"))?;

    Ok(())
}
