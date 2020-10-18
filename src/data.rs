use hex;
use io::Result;
use sha1::{Digest, Sha1};
use std::io::prelude::*;
use std::path::Path;
use std::{fs, io};

pub const GIT_DIR: &'static str = ".ugit";

pub fn init() -> Result<()> {
    std::fs::create_dir(GIT_DIR)?;

    Ok(())
}

pub fn hash_object(pathspecs: Vec<&str>) -> Result<()> {
    for pathspec in pathspecs.iter() {
        let contents = fs::read(pathspec)?;

        let mut hasher = Sha1::new();
        hasher.update(&contents);

        let result = hasher.finalize();

        let path = Path::new(GIT_DIR).join(hex::encode(&result));

        let mut f = fs::File::create(path)?;
        f.write_all(&contents)?;
    }

    Ok(())
}
