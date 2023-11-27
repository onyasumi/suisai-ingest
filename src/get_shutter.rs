use std::error::Error;
use std::path::Path;
use std::process::Command;
use std::io::ErrorKind::NotFound;

pub fn get_shutter(file: &Path) -> Result<String, Box<dyn Error>> {

    if !file.is_file() {
        return Err(format!("Invalid path: {} not exist or is not a file", file.to_str().unwrap_or("")).into())
    }

    let result = String::from_utf8(match Command::new("exiftool")
        .arg("-ShutterCount")
        .arg("-fast1")
        .arg("-s3")
        .arg(file.to_str().unwrap_or(""))
        .output() {
        Ok(result) => result.stdout,
        Err(e) => {
            return if e.kind() == NotFound {
                Err("exiftool not found".into())
            } else {
                Err(e.into())
            }
        }
    })?;


    if result.is_empty() {
        Err("EXIF data does not contain a shutter count".into())
    } else {
        Ok(result.strip_suffix('\n').unwrap_or(&*result).to_string())
    }

}