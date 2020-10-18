use std::io;

pub const GIT_DIR: &'static str = ".ugit";

pub fn init() -> io::Result<()> {
    std::fs::create_dir(GIT_DIR)?;

    Ok(())
}
