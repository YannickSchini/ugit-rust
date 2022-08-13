use std::{fs, path::PathBuf};
use std::fs::File;
use std::io::prelude::*;
use crypto::digest::Digest;
use crypto::sha1::Sha1;

use crate::error::Result;

static UGIT_DIR: &str = ".ugit";

pub fn init() -> Result<()> {
    fs::create_dir(UGIT_DIR)?;
    fs::create_dir(format!("{}/objects", UGIT_DIR))?;
    Ok(())
}

pub fn hash_file(file_name: PathBuf) -> Result<String> {
    let mut f = File::open(file_name)?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;

    let mut hasher = Sha1::new();
    hasher.input_str(&buffer);
    let sha1_identifier = hasher.result_str();

    let mut output_file = File::create(format!("{}/objects/{}", UGIT_DIR, sha1_identifier))?;
    write!(&mut output_file, "{}", &buffer)?;

    Ok(sha1_identifier)
}

pub fn cat_file(oid: String) -> Result<()> {
    Ok(())
}
