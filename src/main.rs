mod metadata;
mod io;

use std::path::Path;
use crate::io::search_directory::search_directory;
use crate::metadata::get_date::get_date;
use crate::metadata::get_shutter::get_shutter;

fn main() {

    let args: Box<[String]> = std::env::args().collect();

    if args.len() < 4 {
        println!("Incorrect Usage: suisai-ingest [source path] [output path] [filename prefix]");
        return;
    }

    let source_dir: &Path = Path::new(&args[1]);
    let dest_dir: &Path = Path::new(&args[2]);

    if !source_dir.exists() {
        println!("Source path does not exist!");
        return;
    }

    if !dest_dir.exists() {
        println!("Destination path does not exist!");
        return;
    }

    println!("Shutter Count: {}", get_shutter(source_dir).unwrap());
    println!("Timestamp: {} {}", get_date(source_dir).unwrap().year, get_date(source_dir).unwrap().month_day);
    search_directory(dest_dir);

}
