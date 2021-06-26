use std::{env, fs, io, path::Path};

//TODO: Without build.rs misuse(???), fix moving input to executable directory without abusing OUT_DIR.
fn main() -> io::Result<()> {
    println!(
        "cargo:warning=OUT_DIR is {:?}",
        env::var("OUT_DIR").unwrap()
    );

    let out_dir = env::var("OUT_DIR").unwrap();
    let inputs_dir_path = Path::new(&out_dir)
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("inputs");

    fs::create_dir_all(&inputs_dir_path)?;
    let exit_path = inputs_dir_path.join("day1.txt");

    fs::copy("input.txt", exit_path)?;
    Ok(())
}
