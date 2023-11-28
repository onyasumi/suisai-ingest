use std::path::Path;
use std::process::Command;
use std::io::ErrorKind::NotFound;
use anyhow::{bail, Result};

pub fn get_shutter(file: &Path) -> Result<String> {

    if !file.is_file() {
        bail!("Invalid path: {} not exist or is not a file", file.to_str().unwrap_or(""))
    }

    let result = String::from_utf8(match Command::new("exiftool")
        .arg("-ShutterCount")
        .arg("-fast1")
        .arg("-s3")
        .arg(file.to_str().unwrap_or(""))
        .output() {
        Ok(result) => result.stdout,
        Err(e) => {
            if e.kind() == NotFound {
                bail!("exiftool not found")
            } else {
                return Err(e.into())
            }
        }
    })?;


    if result.is_empty() {
        bail!("EXIF data does not contain a shutter count")
    } else {
        Ok(result.strip_suffix('\n').unwrap_or(&*result).to_string())
    }

}