use std::ffi::OsStr;
use std::fs::{create_dir_all, rename};
use std::path::Path;
use anyhow::{bail, Result};
use crate::metadata::get_date::get_date;
use crate::metadata::get_shutter::get_shutter;

pub trait Sendable {

    fn send_to(&self, dest: &Path, prefix: &str) -> Result<()>;

}

impl Sendable for Path {
    fn send_to(&self, dest: &Path, prefix: &str) -> Result<()> {

        // Extract date and shutter count
        let date = get_date(self)?;
        let shutter = get_shutter(self)?;

        // Calculate and create destination directory
        let dest_directory = dest.join(date.year).join(date.month_day);
        create_dir_all(&dest_directory)?;

        // Check if destination file exists
        let dest_file = dest_directory.join(format!("{}{}.{}", prefix, shutter, self.extension().unwrap_or(OsStr::new("meow")).to_string_lossy()));
        if dest_file.exists() {
            bail!(format!("File or directory already exists at destination {}", dest_file.to_string_lossy()))
        }

        // Move file
        rename(self, dest_file)?;

        Ok(())

    }

}