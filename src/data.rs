use std::fs;

static UGIT_DIR: &str = ".ugit";

pub fn init() -> std::io::Result<()> {
    fs::create_dir(UGIT_DIR)?;
    Ok(())
}
